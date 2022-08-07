import { resolveMysticLightAddress } from './mysticLightConfig';

resolveMysticLightAddress().then((address) => {
	window.parent.postMessage(address, '*');
});
