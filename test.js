// TODO
// consideration
// do not use lock() directly on mutex. Always use try_lock() to avoid deadlock
// do not use a mutex to share connection. init the pool and use .app_data (https://actix.rs/docs/databases)
// it's worth to use bb8::Pool or just a connection manager it's good enough for this type of application ?


(async () => {
    const fetches = [];
    for(let ii=0; ii<100;ii++){
        fetches.push(fetch("http://localhost:3000/rate", {
            method: "GET",
            headers: { 
                "accept": "application/json"
            }
        }));
    }

    const data = await Promise.all(fetches);
    const xxx = await Promise.all(data.map(x => x.json()))
    console.log(xxx);
})();
