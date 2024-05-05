import { auth } from "$lib/server/auth";
import { fail, redirect } from "@sveltejs/kit";

import type { Actions, PageServerLoad } from "./$types";

export const load: PageServerLoad = async (event) => {
    if (!event.locals.user) {
        redirect(302, "/login");
    }

    const userAttributes = event.locals.user;
    console.log(userAttributes)
    return { 
        id: userAttributes.id,
        isAdmin: userAttributes.isAdmin
    }
};

export const actions: Actions = {
    default: async (event) => {
        if (!event.locals.user) {
            throw fail (401);
        }
    }
};

