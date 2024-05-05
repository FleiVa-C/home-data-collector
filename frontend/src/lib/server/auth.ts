import { Lucia, TimeSpan} from "lucia";
import { dev } from "$app/environment";
import { PrismaAdapter } from "@lucia-auth/adapter-prisma";
import { prisma } from "$lib/server/prisma";

const adapter = new PrismaAdapter(prisma.session, prisma.user);

export const auth = new Lucia(adapter, {
    sessionExpiresIn: new TimeSpan(24, "h"),
    sessionCookie: {
        name: "SessionToken",
        expires: true,
        attributes: {
            secure: !dev
        }
    },
    getUserAttributes: (attributes) => {
        return {
            isAdmin: attributes.isAdmin
        };
    }
});

declare module "lucia" {
    interface Register {
        Lucia: typeof Lucia;
        DatabaseUserAttributes: DatabaseUserAttributes;
    }
}

interface DatabaseUserAttributes {
    isAdmin: boolean;
}
