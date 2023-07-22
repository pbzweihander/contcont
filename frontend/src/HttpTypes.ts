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
  isNsfw: boolean;
  authorHandle: string;
  authorInstance: string;
}

export interface LiteratureMetadata {
  id: number;
  title: string;
  isNsfw: boolean;
  authorHandle: string;
  authorInstance: string;
}

export interface ArtMetadata {
  id: number;
  title: string;
  description: string;
  isNsfw: boolean;
  authorHandle: string;
  authorInstance: string;
}

export interface PostLiteratureReq {
  title: string;
  text: string;
  isNsfw: boolean;
}

export interface PostArtReq {
  title: string;
  description: string;
  isNsfw: boolean;
  file: File;
}

export interface Vote {
  voted: boolean;
  voteCount: number;
}

export interface PostVoteReq {
  id: number;
}
