// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces

import type { PrismaClient } from "@prisma/client";

// and what to do when importing types
declare global {
    namespace App {
	    interface Locals {
            user: import("lucia").User | null;
            session: import("lucia").Session | null;
        }
	// interface PageData {}
	// interface Error {}
	// interface Platform {}
    }
    var prisma: PrismaClient
}

declare module 'd3-scale';
declare module 'chart.js';
export{}
