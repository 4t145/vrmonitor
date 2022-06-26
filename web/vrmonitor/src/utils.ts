export function time_hm(second:number):{hour: number, minute: number} {
    let minute = Math.floor(second/60);
    let hour = Math.floor(minute/60);
    minute = minute - hour*60;
    return {
        hour,
        minute,
    }
}