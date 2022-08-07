interface Options {
	retries?: number;
	initialTimeout?: number;
	timeoutMultiplierFactor?: number;
	onFailedAttempt?: (error: Error) => void | Promise<void>;
}

// eslint-disable-next-line @typescript-eslint/no-empty-function
const noop = () => {};

export const retry = async <T>(
	fn: () => Promise<T>,
	{ retries = 1, initialTimeout = 1000, timeoutMultiplierFactor = 2, onFailedAttempt = noop }: Options = {}
): Promise<T> => {
	let attempt = 0;

	// eslint-disable-next-line no-constant-condition
	while (true) {
		try {
			const result = await fn();

			return result;
		} catch (error) {
			if (attempt >= retries) {
				throw error;
			}

			attempt++;

			await onFailedAttempt(error as Error);

			await new Promise((resolve) => setTimeout(resolve, initialTimeout * Math.pow(timeoutMultiplierFactor, attempt)));
		}
	}
};
