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

QITEM *qHead = NULL;

             
int num_nodes;             
             
//int AdjMatrix[NUM_NODES][NUM_NODES];
int** AdjMatrix;

int g_qCount = 0;

NODE* rgnNodes;

int ch;
int iPrev, iNode;
int i, iCost, iDist;


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
