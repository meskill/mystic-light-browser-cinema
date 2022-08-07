import { gql, GraphQLClient } from 'graphql-request';
import type {
	GetDevicesQuery,
	SetStateForAllMutation,
	SetStateForAllMutationVariables,
	SetStateForSingleLedMutation,
	SetStateForSingleLedMutationVariables,
} from './types/graphql';
import { getPortFromUrl } from './utils/getPortFromUrl';
import { ApiError } from './errors';

const GET_DEVICES_QUERY = gql`
	query GetDevices {
		devices {
			name
			leds {
				name
				state {
					color {
						red
						green
						blue
					}
					bright
					speed
					style
				}
			}
		}
	}
`;

const DEVICES_SET_STATE_ALL = gql`
	mutation SetStateForAll($state: DeviceLedStateInput!) {
		devices {
			leds {
				setState(state: $state)
			}
		}
	}
`;

const DEVICE_SET_STATE_SINGLE = gql`
	mutation SetStateForSingleLed($device_name: String!, $led_name: String!, $state: DeviceLedStateInput!) {
		devices(filter: { names: [$device_name] }) {
			leds(filter: { names: [$led_name] }) {
				setState(state: $state)
			}
		}
	}
`;

export class MysticLightApi {
	private readonly serverUrl: string;
	private readonly client: GraphQLClient;

	private previousState?: GetDevicesQuery;
	constructor(address: string) {
		this.serverUrl = `http://localhost:${getPortFromUrl(address)}`;

		const endpoint = `${this.serverUrl}/mystic_light`;

		this.client = new GraphQLClient(endpoint);
	}

	async healthz() {
		const response = await fetch(`${this.serverUrl}/healthz`);
		const resp = await response.text();

		return resp === 'Ok';
	}

	async turnOffTheLight() {
		this.previousState = await this.client.request<GetDevicesQuery>(GET_DEVICES_QUERY);

		await this.client.request<SetStateForAllMutation, SetStateForAllMutationVariables>(DEVICES_SET_STATE_ALL, {
			state: {
				style: 'NoAnimation',
				color: {
					red: 0,
					green: 0,
					blue: 0,
				},
			},
		});
	}

	async revertPrevStates() {
		if (!this.previousState) {
			throw new ApiError('No previous state');
		}

		await this.client.batchRequests<SetStateForSingleLedMutation, SetStateForSingleLedMutationVariables>(
			this.previousState.devices.flatMap((device) =>
				device.leds.map((led) => {
					return {
						document: DEVICE_SET_STATE_SINGLE,
						variables: {
							device_name: device.name,
							led_name: led.name,
							state: led.state,
						} as SetStateForSingleLedMutationVariables,
					};
				})
			)
		);

		this.previousState = undefined;
	}
}
