declare var self: DedicatedWorkerGlobalScope;
export {};

const helloMessage = {
  hello : 'world',
};

export type HelloMessage = typeof helloMessage;

// Both of these should work.
postMessage(helloMessage);
self.postMessage(helloMessage);
