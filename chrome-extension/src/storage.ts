import { MYSTIC_LIGHT_API_STORAGE_KEY } from './constants/mysticLight';

export const getAddressWithStorage = async (): Promise<string | undefined> => {
	return new Promise<string>((resolve) => {
		chrome.storage.local.get(MYSTIC_LIGHT_API_STORAGE_KEY, (result) => {
			resolve(result?.[MYSTIC_LIGHT_API_STORAGE_KEY]);
		});
	});
};

export const writeAddressIntoStorage = async (address: string): Promise<void> => {
	await chrome.storage.local.set({
		[MYSTIC_LIGHT_API_STORAGE_KEY]: address,
	});
};
