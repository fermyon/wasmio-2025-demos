import { AutoRouter } from 'itty-router';
import { sorting } from '../@spin-deps/wasmio-demo';

let router = AutoRouter();

router
    .get("/", () => new Response("hello universe"))
    .get('/hello/:name', ({ name }) => `Hello, ${name}!`)
    .get("/sorted", () => {
        const input = [2, 3, 12, 334, 3, 676, 1231, 67556];
        const sorted = sorting.sort(Int32Array.from(input));
        return new Response(JSON.stringify(sorted), {
            status: 200,
            headers: {
                "content-type": "application/json"
            }
        })
    })

//@ts-ignore
addEventListener('fetch', (event: FetchEvent) => {
    event.respondWith(router.fetch(event.request));
});
