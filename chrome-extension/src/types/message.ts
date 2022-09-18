type ExtensionMessage<Request extends { type: string }> = Request;

export type ShowAlertExtensionMessage = ExtensionMessage<{ type: 'showAlert'; message: string }>;

export type EnterFullscreenExtensionMessage = ExtensionMessage<{ type: 'enterFullscreen' }>;

export type ExitFullscreenExtensionMessage = ExtensionMessage<{ type: 'exitFullscreen' }>;

export type PingFullscreenExtensionMessage = ExtensionMessage<{ type: 'pingFullscreen' }>;

export type FullscreenExtensionMessages =
	| EnterFullscreenExtensionMessage
	| ExitFullscreenExtensionMessage
	| PingFullscreenExtensionMessage;
