import type { InterfaceData } from "$lib/types";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch }) => {
    const res = await fetch("http://127.0.0.1:8080/v1/interface")
    const item = await res.json() as InterfaceData[];

    return { item };
};
