export interface Event {
  id: number;
  timestamp: string;
  type: EventType;
  data: any;
}

//const data_1 = {
//...events[0],
//connection: {
//domain: 'impierce.com',
//id: 'impierce',
//url: 'https://impierce.com',
//lastConnected: 'n/a',
//},
//title: 'Data shared',
//timestamp: '2023-08-03T12:23:42.749Z',
//credentials: [$state.credentials[0], $state.credentials[1]],
//};

export enum EventType {
  INITIAL_CONNECTION = 'initial_connection',
  CREDENTIAL_OFFER = 'credential_offer',
  LOGIN = 'login',
}
