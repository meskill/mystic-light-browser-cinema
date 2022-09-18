import type { GetDevicesQuery } from './types/graphql';
import { MYSTIC_LIGHT_API_STORAGE_KEY, MYSTIC_LIGHT_PREV_STATE_KEY } from './constants/mysticLight';

export const getAddressWithStorage = async (): Promise<string | undefined> => {
	const result = await chrome.storage.local.get(MYSTIC_LIGHT_API_STORAGE_KEY);

	return result[MYSTIC_LIGHT_API_STORAGE_KEY];
};

export const writeAddressIntoStorage = async (address: string): Promise<void> => {
	await chrome.storage.local.set({
		[MYSTIC_LIGHT_API_STORAGE_KEY]: address,
	});
};

export const getPrevStateStorage = async (): Promise<GetDevicesQuery | undefined> => {
	const result = await chrome.storage.local.get(MYSTIC_LIGHT_PREV_STATE_KEY);

	return result[MYSTIC_LIGHT_PREV_STATE_KEY];
};

export const writePrevStateStorage = async (prevState: GetDevicesQuery) => {
	await chrome.storage.local.set({
		[MYSTIC_LIGHT_PREV_STATE_KEY]: prevState,
	});
};

export const clearPrevStateStorage = async () => {
	await chrome.storage.local.remove(MYSTIC_LIGHT_PREV_STATE_KEY);
};
