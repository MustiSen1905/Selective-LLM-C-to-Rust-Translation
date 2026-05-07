#include <stdio.h>
#include <stdlib.h>

#define NONE                               9999

struct _NODE
{
  int iDist;
  int iPrev;
};
typedef struct _NODE NODE;

struct _QITEM
{
  int iNode;
  int iDist;
  int iPrev;
  struct _QITEM *qNext;
};
typedef struct _QITEM QITEM;

/* These globals are defined in the Rust library (with #[no_mangle]);
   declare them as extern here to avoid duplicate-symbol link errors. */
extern QITEM *qHead;
extern int num_nodes;
extern int** AdjMatrix;
extern int g_qCount;
extern NODE* rgnNodes;
extern int ch;
extern int iPrev, iNode;
extern int i, iCost, iDist;


void print_path (NODE *rgnNodes, int chNode)
;


void enqueue (int iNode, int iDist, int iPrev)
;


void dequeue (int *piNode, int *piDist, int *piPrev)
;


int qcount (void)
{
  return(g_qCount);
}

int dijkstra(int chStart, int chEnd) 
;

int main(int argc, char *argv[]) ;
