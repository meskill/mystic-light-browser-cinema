import type { PlasmoContentScript } from 'plasmo';

import { MysticLightApi } from './api';
import { ApiError } from './errors';
import { getAddressWithStorage, writeAddressIntoStorage } from './storage';
import type { ExtensionMessageShowAlert } from './types/message';
import { retry } from './utils/retry';
import iframeUrl from 'url:./iframe.html';

export const config: PlasmoContentScript = {
	matches: ['<all_urls>'],
};

const getAddressWithIframe = () => {
	return new Promise<string>((resolve) => {
		const iframe = document.createElement('iframe');

		iframe.style.display = 'none';
		iframe.src = iframeUrl;

		const listener = ({ data, origin }: MessageEvent<string>) => {
			if (iframe.src.startsWith(origin)) {
				window.removeEventListener('message', listener);

				writeAddressIntoStorage(data);
				iframe.remove();

				resolve(data);
			}
		};

		window.addEventListener('message', listener);

		document.body.appendChild(iframe);
	});
};

const resolveAddress = async () => {
	const fromStorage = await getAddressWithStorage();

	if (fromStorage) {
		return fromStorage;
	}

	return getAddressWithIframe();
};

let api: MysticLightApi;

const reloadApi = async () => {
	api = new MysticLightApi(await resolveAddress());
};

const resolveApi = async () => {
	if (!api) {
		await reloadApi();
	}
};

const sendEvent = async (apiRequest: () => Promise<void>) => {
	await resolveApi();

	return retry(apiRequest, {
		retries: 3,
		onFailedAttempt: async (error) => {
			// graphql or api error
			if (error instanceof ApiError || 'response' in error) {
				throw error;
			}

			// all other errors
			await getAddressWithIframe();
			await reloadApi();
		},
	});
};

let isInProgress = false;

window.addEventListener('resize', async () => {
	const isFullscreen = !!document.fullscreenElement;

	if (isFullscreen && !isInProgress) {
		isInProgress = true;
		sendEvent(() => api.turnOffTheLight());
	} else if (!isFullscreen && isInProgress) {
		isInProgress = false;
		sendEvent(() => api.revertPrevStates());
	}
});

// restore state if any on browser tab or window close
document.addEventListener('visibilitychange', () => {
	if (document.visibilityState === 'hidden') {
		api?.revertPrevStates();
	}
});

chrome.runtime.onMessage.addListener((request: ExtensionMessageShowAlert['request'], _sender, sendResponse) => {
	if (request.type === 'showAlert') {
		alert(request.message);
	}

	sendResponse();
});
