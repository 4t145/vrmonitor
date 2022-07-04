const PROXY_BASE = 'http://localhost:8001/';


export interface RoomInfo {
    room_id: number,
    short_id: number,
    live_status: number,
    live_time: number
    uid: number
}

export interface UserInfo {
    mid: number,
    name: string,
    face: string,
}


export async function fetch_room_info(room_id: number): Promise<RoomInfo> {

    const url = `${PROXY_BASE}roominfo/${room_id}`;
    
    let resp = await fetch(url, {
        mode: 'cors',
        referrerPolicy: 'same-origin',
    });
    if (resp.ok) {
        return resp.json().then(json => (json as {'data':RoomInfo}) ['data']);
    } else {
        throw new Error(`HttpError${resp.statusText}`);
    }
}

export async function fetch_user_info(uid: number): Promise<UserInfo> {

    const url = `${PROXY_BASE}userinfo/${uid}`;
    
    let resp = await fetch(url, {
        mode: 'cors',
        referrerPolicy: 'same-origin',
    });
    if (resp.ok) {
        return resp.json().then(json => (json as {'data':UserInfo}) ['data']);
    } else {
        throw new Error(`HttpError${resp.statusText}`);
    }
}

