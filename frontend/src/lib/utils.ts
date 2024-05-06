import type { RequestEvent } from "@sveltejs/kit";

export function handleLoginRedirect(event: RequestEvent, msg: string) {
    const redirectTo = event.url.pathname + event.url.search
    return `/login?redirectTo=${redirectTo}&message=${msg}`
}
