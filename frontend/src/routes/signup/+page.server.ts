import { auth } from "$lib/server/auth";
import { fail, redirect } from "@sveltejs/kit";
import { generateId } from "lucia";
import { Argon2id } from "oslo/password";
import { prisma } from "$lib/server/prisma";

import type { Actions } from "./$types";

export const actions: Actions = {
    default: async (event) => {
        const formData = await event.request.formData();
        const username = formData.get("username");
        const password = formData.get("password");
        if (
            typeof username !== "string" ||
            username.length < 3 ||
            username.length > 31 ||
            !/[a-z0-9-_]+$/.test(username)
        ) {
            return fail(400, {
                message: "Invalid Username"
            });
        }
        if (typeof password !== "string" || password.length < 6 || password.length > 255) {
            return fail(400, {
                message: "Invalid password"
            });
        }

        const userId = generateId(15);
        const hashedPassword = await new Argon2id().hash(password);

        await prisma.user.create({
            data: {
                id: userId,
                name: username,
                username: username,
                key: {
                    create: {
                        hashedPassword: hashedPassword
                    }
                },
            },
        });

        const session = await auth.createSession(userId, {});
        const sessionCookie = auth.createSessionCookie(session.id);
        event.cookies.set(sessionCookie.name, sessionCookie.value, {
            path: ".",
            ...sessionCookie.attributes
        });

        redirect(302, "/");
    }
};
