type ExtensionMessage<Request extends { type: string }, Response> = {
	request: Request;
	response: Response;
};

export type ExtensionMessageShowAlert = ExtensionMessage<{ type: 'showAlert'; message: string }, void>;
