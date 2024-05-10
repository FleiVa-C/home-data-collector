import { redirect } from "@sveltejs/kit";
import type { InterfaceData } from "$lib/types";
import type { PageServerLoad } from "./$types";
import { handleLoginRedirect } from "$lib/utils";

export const load: PageServerLoad = async (event) => {
    if (!event.locals.user){
        throw redirect(302, handleLoginRedirect(event, "You need to be signed in to view the Interface."))
    };
    const res = await fetch("http://127.0.0.1:8080/v1/interface")
    const item = await res.json() as InterfaceData[];

    return { 
        interfaces: item
    };

}
