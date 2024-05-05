import { prisma } from "$lib/server/prisma";
import { auth } from "$lib/server/auth";
import { fail, redirect } from "@sveltejs/kit";
import { Argon2id } from "oslo/password";

import type { Actions } from "./$types";

export const actions: Actions = {
    default: async (event) => {
        const formData = await event.request.formData();
        const username = formData.get("username")
        const password = formData.get("password")

        if (
            typeof username !== "string" ||
            username.length < 3 ||
            username.length > 31 ||
            !/^[a-z0-9_-]+$/.test(username)
        ) {
            return fail(400, {
                message: "Invalid Username"
            });
        }

        if (typeof password !== "string" || password.length < 6 || password.length > 255) {
            return fail(400, {
                message: "Invalid Password"
            });
        }

        const existingUser = await prisma.user.findUnique({
            where: {
                username: username.toLowerCase()
            }
        });

        if (!existingUser) {
            return fail(400, {
                message: "Username not found"
            });
        }
        const pw = await prisma.user.findUnique({
            where: {
                username: username.toLowerCase()
            }
        }).key();
        if (pw && pw.length == 1) {
            const validPassword = await new Argon2id().verify(pw[0].hashedPassword, password);
            if (!validPassword) {
                return fail(400, {
                    message: "Incorrect username or password"
                });
            }
        } else {
            return fail(400, {
                message: "Incorrect username or password"
            });
        }

        const session = await auth.createSession(existingUser.id, {});
        const sessionCookie = auth.createSessionCookie(session.id);
        event.cookies.set(sessionCookie.name, sessionCookie.value, {
            path: ".",
            ...sessionCookie.attributes
        });
        redirect(302, "/");
    }
};

