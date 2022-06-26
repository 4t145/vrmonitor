export interface Liver {
    roomid: number,
    uid: number,
    tags: Set<LiverTag>
    // uname: string,
    // avatar: URL,
    // living: boolean
}

export type LiverTag =
     "一期生"
    |"二期生"
    |"三期生"
    |"四期生"
    |"五期生"
    |"六期生"
    |"七期生"
    |"八期生"
    |"九期生"
    |"十期生"
    |"维阿信"


export const FEED_LIST:Liver[] = [
    { roomid: 851181, uid: 59275, tags: new Set(["维阿信"])},
    { roomid: 21457197, uid: 434401868, tags: new Set(["二期生"])},
]