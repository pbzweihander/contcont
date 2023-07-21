export interface User {
  handle: string;
  instance: string;
}

export interface PostOauthAuthorizeReq {
  instance: string;
}

export interface PostOauthAuthorizeResp {
  url: string;
}

export interface GetOpenedResp {
  opened: boolean;
  openAt: string;
  closeAt: string;
}
