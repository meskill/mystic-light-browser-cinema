import { retry } from './retry';

it('should call passed function single time', async () => {
	const fn = jest.fn(async () => 'test response');

	expect(await retry(fn, { retries: 3 })).toBe('test response');

	expect(fn).toBeCalledTimes(1);
});

it('should call function until it is succeeded', async () => {
	let attempt = 0;
	const fn = jest.fn(async () => {
		if (attempt < 2) {
			attempt++;
			throw new Error('test');
		}

		return 'test response';
	});

	const startTime = Date.now();

	await retry(fn, {
		retries: 3,
		initialTimeout: 1000,
		onFailedAttempt: () => {
			Promise.resolve()
				.then(() => Promise.resolve())
				.then(() => {
					jest.runOnlyPendingTimers();
				});
		},
	});

	expect(fn).toHaveBeenCalledTimes(3);
	expect(Date.now() - startTime).toBe(6000);
});

it('should return error from function', async () => {
	const fn = jest.fn(() => {
		throw new Error('test error');
	});
	const startTime = Date.now();

	await expect(
		retry(fn, {
			retries: 3,
			initialTimeout: 1000,
			onFailedAttempt: () => {
				Promise.resolve()
					.then(() => Promise.resolve())
					.then(() => {
						jest.runOnlyPendingTimers();
					});
			},
		})
	).rejects.toThrow('test error');

	expect(fn).toHaveBeenCalledTimes(4);
	expect(Date.now() - startTime).toBe(14000);
});
