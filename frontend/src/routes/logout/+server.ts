import { auth } from "$lib/server/auth";
import { redirect } from "@sveltejs/kit";

import type { RequestHandler } from "./$types";

export const POST: RequestHandler = async ( {locals} ) => {
        const sessionId = locals.session?.id;
        if (sessionId) {
            auth.invalidateSession(sessionId);
        }
        redirect(302, ".");
}
