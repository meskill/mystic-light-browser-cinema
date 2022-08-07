import { MYSTIC_LIGHT_CONFIG_PATH } from './constants/mysticLight';
import type { CoreProps } from './types/mysticLight';

export const resolveMysticLightAddress = async (): Promise<string> => {
	const response = await fetch(MYSTIC_LIGHT_CONFIG_PATH);
	const config: CoreProps = await response.json();

	return config.local_address;
};
