const OPEN_API_BASE = "http://158.247.231.91:443/";

import { DanmakuEvent, Event } from "bilive-danmaku-json";

export type ExtendedEvent<E extends Event> = E & {
    time: number
}

type DanmakuQuery = {
    'user'?: number,
    'time_from': number,
    'time_to': number,
    'limit': number,
    'skip': number
}



function as_query(query: {[key: string]: number|string|boolean}): string {
    let query_string = [];
    for(const key of Object.keys(query)) {
        query_string.push(`${key}=${query[key]}`)
    }
    return query_string.join('&');
}

export async function fetch_danmaku_history(room_id:number, query: DanmakuQuery):Promise<ExtendedEvent<DanmakuEvent>[]> {
    const url =`${OPEN_API_BASE}danmaku/${room_id}?${as_query(query)}`;
    let resp = await fetch(url, {
        mode: 'cors',
        referrerPolicy: 'same-origin',
    });
    if (resp.ok) {
        return resp.json();
    } else {
        throw new Error(`HttpError${resp.statusText}`);
    }
}