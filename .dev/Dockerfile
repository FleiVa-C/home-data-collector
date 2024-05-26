FROM busybox:1.35.0-uclibc as busybox

FROM surrealdb/surrealdb:latest
COPY --from=busybox /bin /bin

ENTRYPOINT /surreal start --log info file:/surreal_db/surreal.db
