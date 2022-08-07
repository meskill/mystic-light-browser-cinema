export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
	ID: string;
	String: string;
	Boolean: boolean;
	Int: number;
	Float: number;
};

/** Represent RGB color */
export type Color = {
	__typename?: 'Color';
	blue: Scalars['Int'];
	green: Scalars['Int'];
	red: Scalars['Int'];
};

/** Represent RGB color */
export type ColorInput = {
	blue: Scalars['Int'];
	green: Scalars['Int'];
	red: Scalars['Int'];
};

/** Represents single hardware MysticLight Device */
export type Device = {
	__typename?: 'Device';
	/** returns device's leds */
	leds: Array<DeviceLed>;
	name: Scalars['String'];
};

/** Represents single hardware MysticLight Device */
export type DeviceLedsArgs = {
	filter?: DeviceLedFilter;
};

/**
 * used for filtering devices.
 * Currently, supports only filtering by name
 */
export type DeviceFilter = {
	names?: InputMaybe<Array<Scalars['String']>>;
};

/** Represents single led of the device */
export type DeviceLed = {
	__typename?: 'DeviceLed';
	maxBright: Scalars['Int'];
	maxSpeed: Scalars['Int'];
	name: Scalars['String'];
	state: DeviceLedState;
	supportedStyles: Array<Scalars['String']>;
};

/**
 * used for filtering device's leds.
 * Currently, supports only filtering by name
 */
export type DeviceLedFilter = {
	names?: InputMaybe<Array<Scalars['String']>>;
};

/** Mutation wrapper for a device led */
export type DeviceLedMutation = {
	__typename?: 'DeviceLedMutation';
	/** updates state for the device led */
	setState: Scalars['Boolean'];
};

/** Mutation wrapper for a device led */
export type DeviceLedMutationSetStateArgs = {
	state: DeviceLedStateInput;
};

/** Represents state of the single led */
export type DeviceLedState = {
	__typename?: 'DeviceLedState';
	/** current brightness of the led (some of the styles do not support this, so there will be fake data in this case) */
	bright: Scalars['Int'];
	/** current color of the led (some of the styles do not support this, so there will be fake data in this case) */
	color: Color;
	/** current speed of the led (some of the styles do not support this, so there will be fake data in this case) */
	speed: Scalars['Int'];
	/** current style of the led */
	style: Scalars['String'];
};

/** Represents state of the single led, but with optional fields */
export type DeviceLedStateInput = {
	/** current brightness of the led (some of the styles do not support this, so there will be fake data in this case) */
	bright?: InputMaybe<Scalars['Int']>;
	/** current color of the led (some of the styles do not support this, so there will be fake data in this case) */
	color?: InputMaybe<ColorInput>;
	/** current speed of the led (some of the styles do not support this, so there will be fake data in this case) */
	speed?: InputMaybe<Scalars['Int']>;
	/** current style of the led */
	style?: InputMaybe<Scalars['String']>;
};

/** Mutation wrapper for a device */
export type DeviceMutation = {
	__typename?: 'DeviceMutation';
	/** returns device's leds wrapped in mutation wrapper */
	leds: Array<DeviceLedMutation>;
};

/** Mutation wrapper for a device */
export type DeviceMutationLedsArgs = {
	filter?: DeviceLedFilter;
};

/** graphql mutation for MysticLightSDK */
export type MysticLightGraphqlMutation = {
	__typename?: 'MysticLightGraphqlMutation';
	/** returns Mystic Light devices wrapped in mutation wrapper */
	devices: Array<DeviceMutation>;
	/** Full reload of Mystic Light SDK to get most-fresh hardware data */
	reload: Scalars['Boolean'];
};

/** graphql mutation for MysticLightSDK */
export type MysticLightGraphqlMutationDevicesArgs = {
	filter?: DeviceFilter;
};

/** graphql query for MysticLightSDK */
export type MysticLightGraphqlQuery = {
	__typename?: 'MysticLightGraphqlQuery';
	/** returns Mystic Light devices */
	devices: Array<Device>;
};

/** graphql query for MysticLightSDK */
export type MysticLightGraphqlQueryDevicesArgs = {
	filter?: DeviceFilter;
};

export type GetDevicesQueryVariables = Exact<{ [key: string]: never }>;

export type GetDevicesQuery = {
	__typename?: 'MysticLightGraphqlQuery';
	devices: Array<{
		__typename?: 'Device';
		name: string;
		leds: Array<{
			__typename?: 'DeviceLed';
			name: string;
			state: {
				__typename?: 'DeviceLedState';
				bright: number;
				speed: number;
				style: string;
				color: { __typename?: 'Color'; red: number; green: number; blue: number };
			};
		}>;
	}>;
};

export type SetStateForAllMutationVariables = Exact<{
	state: DeviceLedStateInput;
}>;

export type SetStateForAllMutation = {
	__typename?: 'MysticLightGraphqlMutation';
	devices: Array<{
		__typename?: 'DeviceMutation';
		leds: Array<{ __typename?: 'DeviceLedMutation'; setState: boolean }>;
	}>;
};

export type SetStateForSingleLedMutationVariables = Exact<{
	device_name: Scalars['String'];
	led_name: Scalars['String'];
	state: DeviceLedStateInput;
}>;

export type SetStateForSingleLedMutation = {
	__typename?: 'MysticLightGraphqlMutation';
	devices: Array<{
		__typename?: 'DeviceMutation';
		leds: Array<{ __typename?: 'DeviceLedMutation'; setState: boolean }>;
	}>;
};
