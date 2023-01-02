export type Method = string;
export type Uri = string;
export type Body = string;
export type Headers = {[key: string]: string};
export type Status = number;
export type UUIDS = string;

export interface RequestStart {
	body: Body,
	headers: Headers,
	id: UUIDS,
	method: Method,
	uri: Uri
}
export interface ResponseComplete {
	body: Body,
	headers: Headers,
	id: UUIDS,
	status_code: Status,
}
export interface ResponseError {
	id: UUIDS,
	error: string
}

export interface Reqeust {
	request: RequestStart,
	response?: ResponseComplete | ResponseError
}
