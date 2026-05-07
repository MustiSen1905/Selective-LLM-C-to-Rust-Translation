/*
 * patricia.c
 *
 * Patricia trie implementation.
 *
 * Functions for inserting nodes, removing nodes, and searching in
 * a Patricia trie designed for IP addresses and netmasks.  A
 * head node must be created with (key,mask) = (0,0).
 *
 * NOTE: The fact that we keep multiple masks per node makes this
 *       more complicated/computationally expensive then a standard
 *       trie.  This is because we need to do longest prefix matching,
 *       which is useful for computer networks, but not as useful
 *       elsewhere.
 *
 * Matthew Smart <mcsmart@eecs.umich.edu>
 *
 * Copyright (c) 2000
 * The Regents of the University of Michigan
 * All rights reserved
 *
 * $Id: patricia.c,v 1.1.1.1 2000/11/06 19:53:17 mguthaus Exp $
 */

#include <stdlib.h>	/* free(), malloc() */
#include <string.h>	/* bcopy() */
#include "patricia.h"


/*
 * Private function used to return whether
 * or not bit 'i' is set in 'key'.
 */
static __inline
unsigned long
bit(int i, unsigned long key)
{
	return key & (1 << (31-i));
}


/*
 * Count the number of masks (and therefore entries)
 * in the Patricia trie.
 */
static int
pat_count(struct ptree *t, int b)
;


/*
 * Private function used for inserting a node recursively.
 */
static struct ptree *
insertR(struct ptree *h, struct ptree *n, int d, struct ptree *p)
;


/*
 * Patricia trie insert.
 *
 * 1) Go down to leaf.
 * 2) Determine longest prefix match with leaf node.
 * 3) Insert new internal node at appropriate location and
 *    attach new external node.
 */
struct ptree *
pat_insert(struct ptree *n, struct ptree *head)
;


/*
 * Remove an entry given a key in a Patricia trie.
 */
int
pat_remove(struct ptree *n, struct ptree *head)
;


/*
 * Find an entry given a key in a Patricia trie.
 */
struct ptree *
pat_search(unsigned long key, struct ptree *head)
;
