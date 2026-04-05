#ifndef USER_AUTH_H
#define USER_AUTH_H

typedef struct {
    char username[16];
    int isAdmin;
    char *session_token;
} User;

User* create_user(const char* name);
void delete_user(User* u);
void elevate_privileges(User* u, int level);

#endif