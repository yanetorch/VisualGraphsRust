

export function __lg  (n)  {
    let cnt = 0;
    let r = 1;
    while( r * 2 <= n ) {
        r *= 2;
        cnt += 1;
    }
    return cnt;
}

export function getRandomFrom2to4() {
    return Math.floor( Math.random()*2 ) + 2;
}