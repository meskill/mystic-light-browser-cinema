import { MysticLightApi } from '~api';
import { waitTabLoaded, waitTabRemoved } from './chrome/tabs';
import { GITHUB_URL } from './constants/info';
import { DISPLAY_NAME } from './constants/info';
import { resolveMysticLightAddress } from './mysticLightConfig';
import { getAddressWithStorage, writeAddressIntoStorage } from './storage';
import type { FullscreenExtensionMessages, ShowAlertExtensionMessage } from './types/message';
import { checkFileSchemeAccess } from './utils/checkFileSchemeAccess';

const ALARM_FULLSCREEN_NAME = 'fullscreen-in-progress';

const openInstructionsTabForNativeApp = async () => {
	const instructionTab = await chrome.tabs.create({
		url: `${GITHUB_URL}#native-app`,
	});

	if (instructionTab.id) {
		await waitTabLoaded(instructionTab);
		await new Promise<void>((resolve) => {
			if (instructionTab.id) {
				chrome.tabs.sendMessage<ShowAlertExtensionMessage>(
					instructionTab.id,
					{
						type: 'showAlert',
						message: `Native app not found.
${DISPLAY_NAME} Extension needs a companion native app to be running.
Make sure you followed the instruction on how to install and run it.`,
					},
					resolve
				);
			}
		});
	}
};

const openInstructionsTabForExtension = async () => {
	const instructionTab = await chrome.tabs.create({
		url: `${GITHUB_URL}#chrome-extension`,
	});

	if (instructionTab.id) {
		await waitTabLoaded(instructionTab);
		await new Promise<void>((resolve) => {
			if (instructionTab.id) {
				chrome.tabs.sendMessage<ShowAlertExtensionMessage>(
					instructionTab.id,
					{
						type: 'showAlert',
						message: `In order to work, ${DISPLAY_NAME} Extension needs permission "Allow access to file URLs".
Please enable this permission on a new tab that will open after you click "OK" on this message`,
					},
					resolve
				);
			}
		});
	}

	const tab = await chrome.tabs.create({
		url: `chrome://extensions/?id=${chrome.runtime.id}`,
	});

	if (!tab.id) {
		throw new Error('Cannot open Extensions Settings Tab');
	}

	await waitTabRemoved(tab);
};

const resolveAddress = async () => {
	const address = await resolveMysticLightAddress();

	await writeAddressIntoStorage(address);
};

const checkApiIsAvailable = async () => {
	const address = await getAddressWithStorage();

	if (!address) {
		return false;
	}

	const api = new MysticLightApi(address);

	return api.healthz().catch(() => false);
};

chrome.runtime.onInstalled.addListener(async () => {
	const hasFileSchemaAccess = await checkFileSchemeAccess();

	if (hasFileSchemaAccess) {
		await resolveAddress();
	} else {
		await openInstructionsTabForExtension();
		await resolveAddress();
	}

	const hasNativeAppRunning = await checkApiIsAvailable();

	if (!hasNativeAppRunning) {
		return openInstructionsTabForNativeApp();
	}
});

chrome.runtime.onConnect.addListener((port) => {
	console.assert(port.name === 'knockknock');
	port.onMessage.addListener((message: FullscreenExtensionMessages) => {
		switch (message.type) {
			case 'enterFullscreen':
			case 'pingFullscreen':
				chrome.alarms.create(ALARM_FULLSCREEN_NAME, {
					delayInMinutes: 3,
				});

				break;
			case 'exitFullscreen':
				chrome.alarms.clear(ALARM_FULLSCREEN_NAME);
				break;
		}
	});
});
