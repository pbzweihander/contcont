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

export interface Literature {
  id: number;
  title: string;
  text: string;
  authorHandle: string;
  authorInstance: string;
}

export interface ArtMetadata {
  id: number;
  title: string;
  authorHandle: string;
  authorInstance: string;
}

export interface PostLiteratureReq {
  title: string;
  text: string;
}

export interface PostArtReq {
  title: string;
  file: File;
}
