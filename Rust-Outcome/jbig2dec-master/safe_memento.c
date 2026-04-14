/* Copyright (C) 2001-2023 Artifex Software, Inc.
   All Rights Reserved.

   This software is provided AS-IS with no warranty, either express or
   implied.

   This software is distributed under license and may not be copied,
   modified or distributed except as expressly authorized under the terms
   of the license contained in the file LICENSE in this distribution.

   Refer to licensing information at http://www.artifex.com or contact
   Artifex Software, Inc.,  39 Mesa Street, Suite 108A, San Francisco,
   CA 94129, USA, for further information.
*/

/* Inspired by Fortify by Simon P Bullen. */

/* Set the following if you're only looking for leaks, not memory overwrites
 * to speed the operation */
/* #define MEMENTO_LEAKONLY */

/* Set the following to keep extra details about the history of blocks */
#define MEMENTO_DETAILS

/* Don't keep blocks around if they'd mean losing more than a quarter of
 * the freelist. */
#define MEMENTO_FREELIST_MAX_SINGLE_BLOCK (MEMENTO_FREELIST_MAX/4)

#define COMPILING_MEMENTO_C

/* SHUT UP, MSVC. I KNOW WHAT I AM DOING. */
#define _CRT_SECURE_NO_WARNINGS

/* We have some GS specific tweaks; more for the GS build environment than
 * anything else. */
/* #define MEMENTO_GS_HACKS */

#ifdef MEMENTO_GS_HACKS
/* For GS we include malloc_.h. Anyone else would just include memento.h */
#include "malloc_.h"
#include "memory_.h"
int atexit(void (*)(void));
#else
#include "memento.h"
#include <stdio.h>
#endif
#ifndef _MSC_VER
#include <stdint.h>
#include <limits.h>
#include <unistd.h>
#endif

#include <stdlib.h>
#include <stdarg.h>
#include <string.h>

#ifdef __ANDROID__
#define MEMENTO_ANDROID
#include <stdio.h>
#endif

/* Hacks to portably print large sizes */
#ifdef _MSC_VER
#define FMTZ "%llu"
#define FMTZ_CAST _int64
#define FMTP "0x%p"
#else
#define FMTZ "%zu"
#define FMTZ_CAST size_t
#define FMTP "%p"
#endif

#define UB(x) ((intptr_t)((x) & 0xFF))
#define B2I(x) (UB(x) | (UB(x)<<8) | (UB(x)<<16) | (UB(x)<<24))
#define B2P(x) ((void *)(B2I(x) | ((B2I(x)<<16)<<16)))
#define MEMENTO_PREFILL_UBYTE ((unsigned char)(MEMENTO_PREFILL))
#define MEMENTO_PREFILL_USHORT (((unsigned short)MEMENTO_PREFILL_UBYTE) | (((unsigned short)MEMENTO_PREFILL_UBYTE)<<8))
#define MEMENTO_PREFILL_UINT (((unsigned int)MEMENTO_PREFILL_USHORT) | (((unsigned int)MEMENTO_PREFILL_USHORT)<<16))
#define MEMENTO_PREFILL_PTR (void *)(((uintptr_t)MEMENTO_PREFILL_UINT) | ((((uintptr_t)MEMENTO_PREFILL_UINT)<<16)<<16))
#define MEMENTO_POSTFILL_UBYTE ((unsigned char)(MEMENTO_POSTFILL))
#define MEMENTO_POSTFILL_USHORT (((unsigned short)MEMENTO_POSTFILL_UBYTE) | (((unsigned short)MEMENTO_POSTFILL_UBYTE)<<8))
#define MEMENTO_POSTFILL_UINT (((unsigned int)MEMENTO_POSTFILL_USHORT) | (((unsigned int)MEMENTO_POSTFILL_USHORT)<<16))
#define MEMENTO_POSTFILL_PTR (void *)(((uintptr_t)MEMENTO_POSTFILL_UINT) | ((((uintptr_t)MEMENTO_POSTFILL_UINT)<<16)<<16))
#define MEMENTO_ALLOCFILL_UBYTE ((unsigned char)(MEMENTO_ALLOCFILL))
#define MEMENTO_ALLOCFILL_USHORT (((unsigned short)MEMENTO_ALLOCFILL_UBYTE) | (((unsigned short)MEMENTO_ALLOCFILL_UBYTE)<<8))
#define MEMENTO_ALLOCFILL_UINT (((unsigned int)MEMENTO_ALLOCFILL_USHORT) | (((unsigned int)MEMENTO_ALLOCFILL_USHORT)<<16))
#define MEMENTO_ALLOCFILL_PTR (void *)(((uintptr_t)MEMENTO_ALLOCFILL_UINT) | ((((uintptr_t)MEMENTO_ALLOCFILL_UINT)<<16)<<16))
#define MEMENTO_FREEFILL_UBYTE ((unsigned char)(MEMENTO_FREEFILL))
#define MEMENTO_FREEFILL_USHORT (((unsigned short)MEMENTO_FREEFILL_UBYTE) | (((unsigned short)MEMENTO_FREEFILL_UBYTE)<<8))
#define MEMENTO_FREEFILL_UINT (((unsigned int)MEMENTO_FREEFILL_USHORT) | (((unsigned int)MEMENTO_FREEFILL_USHORT)<<16))
#define MEMENTO_FREEFILL_PTR (void *)(((uintptr_t)MEMENTO_FREEFILL_UINT) | ((((uintptr_t)MEMENTO_FREEFILL_UINT)<<16)<<16))

#ifdef MEMENTO

#ifndef MEMENTO_CPP_EXTRAS_ONLY

#ifdef MEMENTO_ANDROID
#include <android/log.h>

static char log_buffer[4096];
static int log_fill = 0;

static char log_buffer2[4096];

static int
android_fprintf(FILE *file, const char *fmt, ...)
;

#define fprintf android_fprintf
#define MEMENTO_STACKTRACE_METHOD 3
#endif

/* _WIN64 defined implies _WIN32 will be */
#ifdef _WIN32
#include <windows.h>

static int
windows_fprintf(FILE *file, const char *fmt, ...)
;

#define fprintf windows_fprintf
#endif

#ifndef MEMENTO_STACKTRACE_METHOD
#ifdef __GNUC__
#define MEMENTO_STACKTRACE_METHOD 1
#endif
#ifdef _WIN32
#define MEMENTO_STACKTRACE_METHOD 2
#endif
#endif

#if defined(__linux__)
#define MEMENTO_HAS_FORK
#elif defined(__APPLE__) && defined(__MACH__)
#define MEMENTO_HAS_FORK
#endif

/* Define the underlying allocators, just in case */
void *MEMENTO_UNDERLYING_MALLOC(size_t);
void MEMENTO_UNDERLYING_FREE(void *);
void *MEMENTO_UNDERLYING_REALLOC(void *,size_t);
void *MEMENTO_UNDERLYING_CALLOC(size_t,size_t);

/* And some other standard functions we use. We don't include the header
 * files, just in case they pull in unexpected others. */
int atoi(const char *);
char *getenv(const char *);

/* How far to search for pointers in each block when calculating nestings */
/* mupdf needs at least 34000ish (sizeof(fz_shade))/ */
#define MEMENTO_PTRSEARCH 65536

#ifndef MEMENTO_MAXPATTERN
#define MEMENTO_MAXPATTERN 0
#endif

#ifdef MEMENTO_GS_HACKS
#include "valgrind.h"
#else
#ifdef HAVE_VALGRIND
#include "valgrind/memcheck.h"
#else
#define VALGRIND_MAKE_MEM_NOACCESS(p,s)  do { } while (0==1)
#define VALGRIND_MAKE_MEM_UNDEFINED(p,s)  do { } while (0==1)
#define VALGRIND_MAKE_MEM_DEFINED(p,s)  do { } while (0==1)
#endif
#endif

enum {
    Memento_PreSize  = 16,
    Memento_PostSize = 16
};

/* Some compile time checks */
typedef struct
{
    char MEMENTO_PRESIZE_MUST_BE_A_MULTIPLE_OF_4[Memento_PreSize & 3 ? -1 : 1];
    char MEMENTO_POSTSIZE_MUST_BE_A_MULTIPLE_OF_4[Memento_PostSize & 3 ? -1 : 1];
    char MEMENTO_POSTSIZE_MUST_BE_AT_LEAST_4[Memento_PostSize >= 4 ? 1 : -1];
    char MEMENTO_PRESIZE_MUST_BE_AT_LEAST_4[Memento_PreSize >= 4 ? 1 : -1];
} MEMENTO_SANITY_CHECK_STRUCT;

#define MEMENTO_UINT32 unsigned int
#define MEMENTO_UINT16 unsigned short

#define MEMENTO_PREFILL_UINT32  ((MEMENTO_UINT32)(MEMENTO_PREFILL  | (MEMENTO_PREFILL <<8) | (MEMENTO_PREFILL <<16) |(MEMENTO_PREFILL <<24)))
#define MEMENTO_POSTFILL_UINT16 ((MEMENTO_UINT16)(MEMENTO_POSTFILL | (MEMENTO_POSTFILL<<8)))
#define MEMENTO_POSTFILL_UINT32 ((MEMENTO_UINT32)(MEMENTO_POSTFILL | (MEMENTO_POSTFILL<<8) | (MEMENTO_POSTFILL<<16) |(MEMENTO_POSTFILL<<24)))
#define MEMENTO_FREEFILL_UINT16 ((MEMENTO_UINT16)(MEMENTO_FREEFILL | (MEMENTO_FREEFILL<<8)))
#define MEMENTO_FREEFILL_UINT32 ((MEMENTO_UINT32)(MEMENTO_FREEFILL | (MEMENTO_FREEFILL<<8) | (MEMENTO_FREEFILL<<16) |(MEMENTO_FREEFILL<<24)))

enum {
    Memento_Flag_OldBlock = 1,
    Memento_Flag_HasParent = 2,
    Memento_Flag_BreakOnFree = 4,
    Memento_Flag_BreakOnRealloc = 8,
    Memento_Flag_Freed = 16,
    Memento_Flag_KnownLeak = 32,
    Memento_Flag_Reported = 64
};

enum {
    Memento_EventType_malloc = 0,
    Memento_EventType_calloc = 1,
    Memento_EventType_realloc = 2,
    Memento_EventType_free = 3,
    Memento_EventType_new = 4,
    Memento_EventType_delete = 5,
    Memento_EventType_newArray = 6,
    Memento_EventType_deleteArray = 7,
    Memento_EventType_takeRef = 8,
    Memento_EventType_dropRef = 9,
    Memento_EventType_reference = 10
};

static const char *eventType[] =
{
    "malloc",
    "calloc",
    "realloc",
    "free",
    "new",
    "delete",
    "new[]",
    "delete[]",
    "takeRef",
    "dropRef",
    "reference"
};

/* When we list leaked blocks at the end of execution, we search for pointers
 * between blocks in order to be able to give a nice nested view.
 * Unfortunately, if you have are running your own allocator (such as
 * postscript's chunk allocator) you can often find that the header of the
 * block always contains pointers to next or previous blocks. This tends to
 * mean the nesting displayed is "uninteresting" at best :)
 *
 * As a hack to get around this, we have a define MEMENTO_SKIP_SEARCH that
 * indicates how many bytes to skip over at the start of the chunk.
 * This may cause us to miss true nestings, but such is life...
 */
#ifndef MEMENTO_SEARCH_SKIP
#ifdef MEMENTO_GS_HACKS
#define MEMENTO_SEARCH_SKIP (2*sizeof(void *))
#else
#define MEMENTO_SEARCH_SKIP 0
#endif
#endif

#define MEMENTO_CHILD_MAGIC   ((Memento_BlkHeader *)('M' | ('3' << 8) | ('m' << 16) | ('3' << 24)))
#define MEMENTO_SIBLING_MAGIC ((Memento_BlkHeader *)('n' | ('t' << 8) | ('0' << 16) | ('!' << 24)))

#ifdef MEMENTO_DETAILS
typedef struct Memento_BlkDetails Memento_BlkDetails;

struct Memento_BlkDetails
{
    Memento_BlkDetails *next;
    char                type;
    char                count;
    int                 sequence;
    void               *stack[1];
};
#endif /* MEMENTO_DETAILS */

typedef struct Memento_BlkHeader Memento_BlkHeader;

struct Memento_BlkHeader
{
    size_t               rawsize;
    int                  sequence;
    int                  lastCheckedOK;
    int                  flags;
    Memento_BlkHeader   *next;
    Memento_BlkHeader   *prev; /* Reused as 'parent' when printing nested list */

    const char          *label;

    /* Entries for nesting display calculations. Set to magic
     * values at all other time.  */
    Memento_BlkHeader   *child;
    Memento_BlkHeader   *sibling;

#ifdef MEMENTO_DETAILS
    Memento_BlkDetails  *details;
    Memento_BlkDetails **details_tail;
#endif

    char                 preblk[Memento_PreSize];
};

/* In future this could (should) be a smarter data structure, like, say,
 * splay trees. For now, we use a list.
 */
typedef struct Memento_Blocks
{
    Memento_BlkHeader *head;
    Memento_BlkHeader *tail;
} Memento_Blocks;

/* What sort of Mutex should we use? */
#ifdef MEMENTO_LOCKLESS
typedef int Memento_mutex;

static void Memento_initMutex(Memento_mutex *m)
{
    (void)m;
}

#define MEMENTO_DO_LOCK() do { } while (0)
#define MEMENTO_DO_UNLOCK() do { } while (0)

#else
#if defined(_WIN32) || defined(_WIN64)
/* Windows */
typedef CRITICAL_SECTION Memento_mutex;

static void Memento_initMutex(Memento_mutex *m)
{
    InitializeCriticalSection(m);
}

#define MEMENTO_DO_LOCK() \
    EnterCriticalSection(&memento.mutex)
#define MEMENTO_DO_UNLOCK() \
    LeaveCriticalSection(&memento.mutex)

#else
#include <pthread.h>
typedef pthread_mutex_t Memento_mutex;

static void Memento_initMutex(Memento_mutex *m)
;

#define MEMENTO_DO_LOCK() \
    pthread_mutex_lock(&memento.mutex)
#define MEMENTO_DO_UNLOCK() \
    pthread_mutex_unlock(&memento.mutex)

#endif
#endif

/* And our global structure */
static struct {
    int            inited;
    Memento_Blocks used;
    Memento_Blocks free;
    size_t         freeListSize;
    int            sequence;
    int            paranoia;
    int            paranoidAt;
    int            countdown;
    int            lastChecked;
    int            breakAt;
    int            failAt;
    int            failing;
    int            nextFailAt;
    int            squeezeAt;
    int            squeezing;
    int            segv;
    int            pattern;
    int            nextPattern;
    int            patternBit;
    int            leaking;
    size_t         maxMemory;
    size_t         alloc;
    size_t         peakAlloc;
    size_t         totalAlloc;
    size_t         numMallocs;
    size_t         numFrees;
    size_t         numReallocs;
    Memento_mutex  mutex;
} memento;

#define MEMENTO_EXTRASIZE (sizeof(Memento_BlkHeader) + Memento_PostSize)

/* Round up size S to the next multiple of N (where N is a power of 2) */
#define MEMENTO_ROUNDUP(S,N) ((S + N-1)&~(N-1))

#define MEMBLK_SIZE(s) MEMENTO_ROUNDUP(s + MEMENTO_EXTRASIZE, MEMENTO_MAXALIGN)

#define MEMBLK_FROMBLK(B)   (&((Memento_BlkHeader*)(void *)(B))[-1])
#define MEMBLK_TOBLK(B)     ((void*)(&((Memento_BlkHeader*)(void*)(B))[1]))
#define MEMBLK_POSTPTR(B) \
          (&((unsigned char *)(void *)(B))[(B)->rawsize + sizeof(Memento_BlkHeader)])

enum
{
    SkipStackBackTraceLevels = 4
};

#if defined(MEMENTO_STACKTRACE_METHOD) && MEMENTO_STACKTRACE_METHOD == 1
extern size_t backtrace(void **, int);
extern void backtrace_symbols_fd(void **, size_t, int);
extern char **backtrace_symbols(void **, size_t);

#define MEMENTO_BACKTRACE_MAX 256
static void (*print_stack_value)(void *address);

/* Libbacktrace gubbins - relies on us having libdl to load the .so */
#ifdef HAVE_LIBDL
#include <dlfcn.h>

typedef void (*backtrace_error_callback) (void *data, const char *msg, int errnum);

typedef struct backtrace_state *(*backtrace_create_state_type)(
    const char *filename, int threaded,
    backtrace_error_callback error_callback, void *data);

typedef int (*backtrace_full_callback) (void *data, uintptr_t pc,
                                        const char *filename, int lineno,
                                        const char *function);

typedef int (*backtrace_pcinfo_type)(struct backtrace_state *state,
                                     uintptr_t pc,
                                     backtrace_full_callback callback,
                                     backtrace_error_callback error_callback,
                                     void *data);

typedef void (*backtrace_syminfo_callback) (void *data, uintptr_t pc,
                                            const char *symname,
                                            uintptr_t symval,
                                            uintptr_t symsize);

typedef int (*backtrace_syminfo_type)(struct backtrace_state *state,
                                      uintptr_t addr,
                                      backtrace_syminfo_callback callback,
                                      backtrace_error_callback error_callback,
                                      void *data);

static backtrace_syminfo_type backtrace_syminfo;
static backtrace_create_state_type backtrace_create_state;
static backtrace_pcinfo_type backtrace_pcinfo;
static struct backtrace_state *my_backtrace_state;
static void *libbt;
static char backtrace_exe[4096];
static void *current_addr;

static void error2_cb(void *data, const char *msg, int errnum)
;

static void syminfo_cb(void *data, uintptr_t pc, const char *symname, uintptr_t symval, uintptr_t symsize)
;

static void error_cb(void *data, const char *msg, int errnum)
;

static int full_cb(void *data, uintptr_t pc, const char *fname, int line, const char *fn)
;

static void print_stack_libbt(void *addr)
;

static void print_stack_libbt_failed(void *addr)
;

static int init_libbt(void)
;
#endif

static void print_stack_default(void *addr)
;

static void Memento_initStacktracer(void)
{
    print_stack_value = print_stack_default;
}

static int Memento_getStacktrace(void **stack, int *skip)
{
    size_t num;

    num = backtrace(&stack[0], MEMENTO_BACKTRACE_MAX);

    *skip = SkipStackBackTraceLevels;
    if (num <= SkipStackBackTraceLevels)
        return 0;
    return (int)(num-SkipStackBackTraceLevels);
}

static void Memento_showStacktrace(void **stack, int numberOfFrames)
{
    int i;

    for (i = 0; i < numberOfFrames; i++)
    {
        print_stack_value(stack[i]);
    }
}
#elif defined(MEMENTO_STACKTRACE_METHOD) && MEMENTO_STACKTRACE_METHOD == 2
#include <Windows.h>

/* We use DbgHelp.dll rather than DbgHelp.lib. This avoids us needing
 * extra link time complications, and enables us to fall back gracefully
 * if the DLL cannot be found.
 *
 * To achieve this we have our own potted versions of the required types
 * inline here.
 */
#ifdef _WIN64
typedef DWORD64 DWORD_NATIVESIZED;
#else
typedef DWORD DWORD_NATIVESIZED;
#endif

#define MEMENTO_BACKTRACE_MAX 64

typedef USHORT (__stdcall *My_CaptureStackBackTraceType)(__in ULONG, __in ULONG, __out PVOID*, __out_opt PULONG);

typedef struct MY_IMAGEHLP_LINE {
    DWORD    SizeOfStruct;
    PVOID    Key;
    DWORD    LineNumber;
    PCHAR    FileName;
    DWORD_NATIVESIZED    Address;
} MY_IMAGEHLP_LINE, *MY_PIMAGEHLP_LINE;

typedef BOOL (__stdcall *My_SymGetLineFromAddrType)(HANDLE hProcess, DWORD_NATIVESIZED dwAddr, PDWORD pdwDisplacement, MY_PIMAGEHLP_LINE Line);

typedef struct MY_SYMBOL_INFO {
    ULONG       SizeOfStruct;
    ULONG       TypeIndex;        // Type Index of symbol
    ULONG64     Reserved[2];
    ULONG       info;
    ULONG       Size;
    ULONG64     ModBase;          // Base Address of module containing this symbol
    ULONG       Flags;
    ULONG64     Value;            // Value of symbol, ValuePresent should be 1
    ULONG64     Address;          // Address of symbol including base address of module
    ULONG       Register;         // register holding value or pointer to value
    ULONG       Scope;            // scope of the symbol
    ULONG       Tag;              // pdb classification
    ULONG       NameLen;          // Actual length of name
    ULONG       MaxNameLen;
    CHAR        Name[1];          // Name of symbol
} MY_SYMBOL_INFO, *MY_PSYMBOL_INFO;

typedef BOOL (__stdcall *My_SymFromAddrType)(HANDLE hProcess, DWORD64 Address, PDWORD64 Displacement, MY_PSYMBOL_INFO Symbol);
typedef BOOL (__stdcall *My_SymInitializeType)(HANDLE hProcess, PSTR UserSearchPath, BOOL fInvadeProcess);

static My_CaptureStackBackTraceType Memento_CaptureStackBackTrace;
static My_SymGetLineFromAddrType Memento_SymGetLineFromAddr;
static My_SymFromAddrType Memento_SymFromAddr;
static My_SymInitializeType Memento_SymInitialize;
static HANDLE Memento_process;

static void Memento_initStacktracer(void)
{
    HMODULE mod = LoadLibrary("kernel32.dll");

    if (mod == NULL)
        return;
    Memento_CaptureStackBackTrace = (My_CaptureStackBackTraceType)(GetProcAddress(mod, "RtlCaptureStackBackTrace"));
    if (Memento_CaptureStackBackTrace == NULL)
        return;
    mod = LoadLibrary("Dbghelp.dll");
    if (mod == NULL) {
        Memento_CaptureStackBackTrace = NULL;
        return;
    }
    Memento_SymGetLineFromAddr =
            (My_SymGetLineFromAddrType)(GetProcAddress(mod,
#ifdef _WIN64
                                                       "SymGetLineFromAddr64"
#else
                                                       "SymGetLineFromAddr"
#endif
                                        ));
    if (Memento_SymGetLineFromAddr == NULL) {
        Memento_CaptureStackBackTrace = NULL;
        return;
    }
    Memento_SymFromAddr = (My_SymFromAddrType)(GetProcAddress(mod, "SymFromAddr"));
    if (Memento_SymFromAddr == NULL) {
        Memento_CaptureStackBackTrace = NULL;
        return;
    }
    Memento_SymInitialize = (My_SymInitializeType)(GetProcAddress(mod, "SymInitialize"));
    if (Memento_SymInitialize == NULL) {
        Memento_CaptureStackBackTrace = NULL;
        return;
    }
    Memento_process = GetCurrentProcess();
    Memento_SymInitialize(Memento_process, NULL, TRUE);
}

static int Memento_getStacktrace(void **stack, int *skip)
{
    if (Memento_CaptureStackBackTrace == NULL)
        return 0;

    *skip = 0;
    /* Limit us to 63 levels due to windows bug */
    return Memento_CaptureStackBackTrace(SkipStackBackTraceLevels, 63-SkipStackBackTraceLevels, stack, NULL);
}

static void Memento_showStacktrace(void **stack, int numberOfFrames)
{
    MY_IMAGEHLP_LINE line;
    int i;
    char symbol_buffer[sizeof(MY_SYMBOL_INFO) + 1024 + 1];
    MY_SYMBOL_INFO *symbol = (MY_SYMBOL_INFO *)symbol_buffer;

    symbol->MaxNameLen = 1024;
    symbol->SizeOfStruct = sizeof(MY_SYMBOL_INFO);
    line.SizeOfStruct = sizeof(MY_IMAGEHLP_LINE);
    for (i = 0; i < numberOfFrames; i++)
    {
        DWORD64 dwDisplacement64;
        DWORD dwDisplacement;
        Memento_SymFromAddr(Memento_process, (DWORD64)(stack[i]), &dwDisplacement64, symbol);
        Memento_SymGetLineFromAddr(Memento_process, (DWORD_NATIVESIZED)(stack[i]), &dwDisplacement, &line);
        fprintf(stderr, "    %s in %s:%d\n", symbol->Name, line.FileName, line.LineNumber);
    }
}
#elif defined(MEMENTO_STACKTRACE_METHOD) && MEMENTO_STACKTRACE_METHOD == 3

#include <unwind.h>
#include <dlfcn.h>

/* From cxxabi.h */
extern char* __cxa_demangle(const char* mangled_name,
                            char*       output_buffer,
                            size_t*     length,
                            int*        status);

static void Memento_initStacktracer(void)
{
}

#define MEMENTO_BACKTRACE_MAX 256

typedef struct
{
    int count;
    void **addr;
} my_unwind_details;

static _Unwind_Reason_Code unwind_populate_callback(struct _Unwind_Context *context,
                                                    void *arg)
;

static int Memento_getStacktrace(void **stack, int *skip)
{
    my_unwind_details uw = { 0, stack };

    *skip = 0;

    /* Collect the backtrace. Deliberately only unwind once,
     * and avoid using malloc etc until this completes just
     * in case. */
    _Unwind_Backtrace(unwind_populate_callback, &uw);
    if (uw.count <= SkipStackBackTraceLevels)
        return 0;

    *skip = SkipStackBackTraceLevels;
    return uw.count-SkipStackBackTraceLevels;
}

static void Memento_showStacktrace(void **stack, int numberOfFrames)
{
    int i;

    for (i = 0; i < numberOfFrames; i++)
    {
        Dl_info info;
        if (dladdr(stack[i], &info))
        {
            int status = 0;
            const char *sym = info.dli_sname ? info.dli_sname : "<unknown>";
            char *demangled = __cxa_demangle(sym, NULL, 0, &status);
            int offset = stack[i] - info.dli_saddr;
            fprintf(stderr, "    ["FMTP"]%s(+0x%x)\n", stack[i], demangled && status == 0 ? demangled : sym, offset);
            free(demangled);
        }
        else
        {
            fprintf(stderr, "    ["FMTP"]\n", stack[i]);
        }
    }
}

#else
static void Memento_initStacktracer(void)
;

static int Memento_getStacktrace(void **stack, int *skip)
;

static void Memento_showStacktrace(void **stack, int numberOfFrames)
;
#endif /* MEMENTO_STACKTRACE_METHOD */

#ifdef MEMENTO_DETAILS
static void Memento_storeDetails(Memento_BlkHeader *head, int type)
;
#endif

void (Memento_bt)(void)
{
#ifdef MEMENTO_STACKTRACE_METHOD
    void *stack[MEMENTO_BACKTRACE_MAX];
    int count;
    int skip;

    count = Memento_getStacktrace(stack, &skip);
    Memento_showStacktrace(&stack[skip-2], count-skip+2);
#endif
}

static void Memento_bt_internal(int skip2)
;

static int Memento_checkAllMemoryLocked(void);

void Memento_breakpoint(void)
;

static void Memento_init(void);

#define MEMENTO_LOCK() \
do { if (!memento.inited) Memento_init(); MEMENTO_DO_LOCK(); } while (0)

#define MEMENTO_UNLOCK() \
do { MEMENTO_DO_UNLOCK(); } while (0)

/* Do this as a macro to prevent another level in the callstack,
 * which is annoying while stepping. */
#define Memento_breakpointLocked() \
do { MEMENTO_UNLOCK(); Memento_breakpoint(); MEMENTO_LOCK(); } while (0)

static void Memento_addBlockHead(Memento_Blocks    *blks,
                                 Memento_BlkHeader *b,
                                 int                type)
;

static void Memento_addBlockTail(Memento_Blocks    *blks,
                                 Memento_BlkHeader *b,
                                 int                type)
;

typedef struct BlkCheckData {
    int found;
    int preCorrupt;
    int postCorrupt;
    int freeCorrupt;
    size_t index;
} BlkCheckData;

#ifndef MEMENTO_LEAKONLY
static int Memento_Internal_checkAllocedBlock(Memento_BlkHeader *b, void *arg)
;

static int Memento_Internal_checkFreedBlock(Memento_BlkHeader *b, void *arg)
;
#endif /* MEMENTO_LEAKONLY */

static void Memento_removeBlock(Memento_Blocks    *blks,
                                Memento_BlkHeader *b)
;

static void free_block(Memento_BlkHeader *head)
;

static int Memento_Internal_makeSpace(size_t space)
;

static int Memento_appBlocks(Memento_Blocks *blks,
                             int             (*app)(Memento_BlkHeader *,
                                                    void *),
                             void           *arg)
{
    Memento_BlkHeader *head = blks->head;
    Memento_BlkHeader *next;
    int                result;
    while (head) {
        VALGRIND_MAKE_MEM_DEFINED(head, sizeof(Memento_BlkHeader));
        VALGRIND_MAKE_MEM_DEFINED(MEMBLK_TOBLK(head),
                                  head->rawsize + Memento_PostSize);
        result = app(head, arg);
        next = head->next;
        VALGRIND_MAKE_MEM_NOACCESS(MEMBLK_POSTPTR(head), Memento_PostSize);
        VALGRIND_MAKE_MEM_NOACCESS(head, sizeof(Memento_BlkHeader));
        if (result)
            return result;
        head = next;
    }
    return 0;
}

#ifndef MEMENTO_LEAKONLY
/* Distrustful - check the block is a real one */
static int Memento_appBlockUser(Memento_Blocks    *blks,
                                int                (*app)(Memento_BlkHeader *,
                                                          void *),
                                void              *arg,
                                Memento_BlkHeader *b)
{
    Memento_BlkHeader *head = blks->head;
    Memento_BlkHeader *next;
    int                result;
    while (head && head != b) {
        VALGRIND_MAKE_MEM_DEFINED(head, sizeof(Memento_BlkHeader));
        next = head->next;
       VALGRIND_MAKE_MEM_NOACCESS(MEMBLK_POSTPTR(head), Memento_PostSize);
        head = next;
    }
    if (head == b) {
        VALGRIND_MAKE_MEM_DEFINED(head, sizeof(Memento_BlkHeader));
        VALGRIND_MAKE_MEM_DEFINED(MEMBLK_TOBLK(head),
                                  head->rawsize + Memento_PostSize);
        result = app(head, arg);
        VALGRIND_MAKE_MEM_NOACCESS(MEMBLK_POSTPTR(head), Memento_PostSize);
        VALGRIND_MAKE_MEM_NOACCESS(head, sizeof(Memento_BlkHeader));
        return result;
    }
    return 0;
}

static int Memento_appBlock(Memento_Blocks    *blks,
                            int                (*app)(Memento_BlkHeader *,
                                                      void *),
                            void              *arg,
                            Memento_BlkHeader *b)
{
    int result;
    VALGRIND_MAKE_MEM_DEFINED(b, sizeof(Memento_BlkHeader));
    VALGRIND_MAKE_MEM_DEFINED(MEMBLK_TOBLK(b),
                              b->rawsize + Memento_PostSize);
    result = app(b, arg);
    VALGRIND_MAKE_MEM_NOACCESS(MEMBLK_POSTPTR(b), Memento_PostSize);
    VALGRIND_MAKE_MEM_NOACCESS(b, sizeof(Memento_BlkHeader));
    return result;
}
#endif /* MEMENTO_LEAKONLY */

static int showBlock(Memento_BlkHeader *b, int space)
;

static void blockDisplay(Memento_BlkHeader *b, int n)
;

static int Memento_listBlock(Memento_BlkHeader *b,
                             void              *arg)
;

static void doNestedDisplay(Memento_BlkHeader *b,
                            int depth)
;

static int ptrcmp(const void *a_, const void *b_)
;

static
int Memento_listBlocksNested(void)
;

void Memento_listBlocks(void)
;

static int Memento_listNewBlock(Memento_BlkHeader *b,
                                void              *arg)
;

void Memento_listNewBlocks(void)
;

static void Memento_endStats(void)
;

void Memento_stats(void)
{
    MEMENTO_LOCK();
    fprintf(stderr, "Current memory malloced = "FMTZ" bytes\n", (FMTZ_CAST)memento.alloc);
    Memento_endStats();
    MEMENTO_UNLOCK();
}

#ifdef MEMENTO_DETAILS
static int showInfo(Memento_BlkHeader *b, void *arg)
;
#endif

void Memento_listBlockInfo(void)
{
#ifdef MEMENTO_DETAILS
    MEMENTO_LOCK();
    fprintf(stderr, "Details of allocated blocks:\n");
    Memento_appBlocks(&memento.used, showInfo, NULL);
    MEMENTO_UNLOCK();
#endif
}

static int Memento_nonLeakBlocksLeaked(void)
;

void Memento_fin(void)
{
    Memento_checkAllMemory();
    if (!memento.segv)
    {
        Memento_endStats();
        if (Memento_nonLeakBlocksLeaked()) {
            Memento_listBlocks();
#ifdef MEMENTO_DETAILS
            fprintf(stderr, "\n");
            Memento_listBlockInfo();
#endif
            Memento_breakpoint();
        }
    }
    if (memento.squeezing) {
        if (memento.pattern == 0)
            fprintf(stderr, "Memory squeezing @ %d complete%s\n", memento.squeezeAt, memento.segv ? " (with SEGV)" : "");
        else
            fprintf(stderr, "Memory squeezing @ %d (%d) complete%s\n", memento.squeezeAt, memento.pattern, memento.segv ? " (with SEGV)" : "");
    } else if (memento.segv) {
        fprintf(stderr, "Memento completed (with SEGV)\n");
    }
    if (memento.failing)
    {
        fprintf(stderr, "MEMENTO_FAILAT=%d\n", memento.failAt);
        fprintf(stderr, "MEMENTO_PATTERN=%d\n", memento.pattern);
    }
    if (memento.nextFailAt != 0)
    {
        fprintf(stderr, "MEMENTO_NEXTFAILAT=%d\n", memento.nextFailAt);
        fprintf(stderr, "MEMENTO_NEXTPATTERN=%d\n", memento.nextPattern);
    }
}

static void Memento_init(void)
;

typedef struct findBlkData {
    void              *addr;
    Memento_BlkHeader *blk;
    int                flags;
} findBlkData;

static int Memento_containsAddr(Memento_BlkHeader *b,
                                void *arg)
;

void Memento_info(void *addr)
;

#ifdef MEMENTO_HAS_FORK
#include <unistd.h>
#include <sys/wait.h>
#include <time.h>
#ifdef MEMENTO_STACKTRACE_METHOD
#if MEMENTO_STACKTRACE_METHOD == 1
#include <signal.h>
#endif
#endif

/* FIXME: Find some portable way of getting this */
/* MacOSX has 10240, Ubuntu seems to have 256 */
#ifndef OPEN_MAX
#define OPEN_MAX 10240
#endif

/* stashed_map[j] = i means that file descriptor i-1 was duplicated to j */
int stashed_map[OPEN_MAX];

static void Memento_signal(int sig)
{
    (void)sig;
    fprintf(stderr, "SEGV at:\n");
    memento.segv = 1;
    Memento_bt_internal(0);

    exit(1);
}

static int squeeze(void)
{
    pid_t pid;
    int i, status;

    if (memento.patternBit < 0)
        return 1;
    if (memento.squeezing && memento.patternBit >= MEMENTO_MAXPATTERN)
        return 1;

    if (memento.patternBit == 0)
        memento.squeezeAt = memento.sequence;

    if (!memento.squeezing) {
        fprintf(stderr, "Memory squeezing @ %d\n", memento.squeezeAt);
    } else
        fprintf(stderr, "Memory squeezing @ %d (%x,%x)\n", memento.squeezeAt, memento.pattern, memento.patternBit);

    /* When we fork below, the child is going to snaffle all our file pointers
     * and potentially corrupt them. Let's make copies of all of them before
     * we fork, so we can restore them when we restart. */
    for (i = 0; i < OPEN_MAX; i++) {
        if (stashed_map[i] == 0) {
            int j = dup(i);
            stashed_map[j] = i+1;
        }
    }

    fprintf(stderr, "Failing at:\n");
    Memento_bt_internal(2);
    pid = fork();
    if (pid == 0) {
        /* Child */
        signal(SIGSEGV, Memento_signal);
        /* In the child, we always fail the next allocation. */
        if (memento.patternBit == 0) {
            memento.patternBit = 1;
        } else
            memento.patternBit <<= 1;
        memento.squeezing = 1;
        return 1;
    }

    /* In the parent if we hit another allocation, pass it (and record the
     * fact we passed it in the pattern. */
    memento.pattern |= memento.patternBit;
    memento.patternBit <<= 1;

    /* Wait for pid to finish, with a timeout. */
    {
        struct timespec tm = { 0, 10 * 1000 * 1000 }; /* 10ms = 100th sec */
        int timeout = 30 * 1000 * 1000; /* time out in microseconds! */
        while (waitpid(pid, &status, WNOHANG) == 0) {
            nanosleep(&tm, NULL);
            timeout -= (tm.tv_nsec/1000);
            tm.tv_nsec *= 2;
            if (tm.tv_nsec > 999999999)
                tm.tv_nsec = 999999999;
            if (timeout <= 0) {
                char text[32];
                fprintf(stderr, "Child is taking a long time to die. Killing it.\n");
                sprintf(text, "kill %d", pid);
                system(text);
                break;
            }
        }
    }

    if (status != 0) {
        fprintf(stderr, "Child status=%d\n", status);
    }

    /* Put the files back */
    for (i = 0; i < OPEN_MAX; i++) {
        if (stashed_map[i] != 0) {
            dup2(i, stashed_map[i]-1);
            close(i);
            stashed_map[i] = 0;
        }
    }

    return 0;
}
#else
#include <signal.h>

static void Memento_signal(int sig)
;

static int squeeze(void)
;
#endif

static void Memento_startFailing(void)
;

static int Memento_event(void)
;

int Memento_sequence(void)
{
    return memento.sequence;
}

int Memento_breakAt(int event)
{
    MEMENTO_LOCK();
    memento.breakAt = event;
    MEMENTO_UNLOCK();
    return event;
}

static void *safe_find_block(void *ptr)
{
    Memento_BlkHeader *block;
    int valid;

    if (ptr == NULL)
        return NULL;

    block = MEMBLK_FROMBLK(ptr);
    /* Sometimes wrapping allocators can mean Memento_label
     * is called with a value within the block, rather than
     * at the start of the block. If we detect this, find it
     * the slow way. */
    VALGRIND_MAKE_MEM_DEFINED(&block->child, sizeof(block->child));
    VALGRIND_MAKE_MEM_DEFINED(&block->sibling, sizeof(block->sibling));
    valid = (block->child == MEMENTO_CHILD_MAGIC &&
             block->sibling == MEMENTO_SIBLING_MAGIC);
    VALGRIND_MAKE_MEM_NOACCESS(&block->child, sizeof(block->child));
    VALGRIND_MAKE_MEM_NOACCESS(&block->sibling, sizeof(block->sibling));
    if (!valid)
    {
        findBlkData data;

        data.addr  = ptr;
        data.blk   = NULL;
        data.flags = 0;
        Memento_appBlocks(&memento.used, Memento_containsAddr, &data);
        if (data.blk == NULL)
            return NULL;
        block = data.blk;
    }
    return block;
}

void *Memento_label(void *ptr, const char *label)
{
    Memento_BlkHeader *block;

    if (ptr == NULL)
        return NULL;
    MEMENTO_LOCK();
    block = safe_find_block(ptr);
    if (block != NULL)
    {
        VALGRIND_MAKE_MEM_DEFINED(&block->label, sizeof(block->label));
        block->label = label;
        VALGRIND_MAKE_MEM_NOACCESS(&block->label, sizeof(block->label));
    }
    MEMENTO_UNLOCK();
    return ptr;
}

void Memento_tick(void)
{
    MEMENTO_LOCK();
    if (Memento_event()) Memento_breakpointLocked();
    MEMENTO_UNLOCK();
}

static int Memento_failThisEventLocked(void)
;

int Memento_failThisEvent(void)
{
    int ret;

    if (!memento.inited)
        Memento_init();

    MEMENTO_LOCK();
    ret = Memento_failThisEventLocked();
    MEMENTO_UNLOCK();
    return ret;
}

static void *do_malloc(size_t s, int eventType)
{
    Memento_BlkHeader *memblk;
    size_t             smem = MEMBLK_SIZE(s);

    if (Memento_failThisEventLocked())
        return NULL;

    if (s == 0)
        return NULL;

    memento.numMallocs++;

    if (memento.maxMemory != 0 && memento.alloc + s > memento.maxMemory)
        return NULL;

    memblk = MEMENTO_UNDERLYING_MALLOC(smem);
    if (memblk == NULL)
        return NULL;

    memento.alloc      += s;
    memento.totalAlloc += s;
    if (memento.peakAlloc < memento.alloc)
        memento.peakAlloc = memento.alloc;
#ifndef MEMENTO_LEAKONLY
    memset(MEMBLK_TOBLK(memblk), MEMENTO_ALLOCFILL, s);
#endif
    memblk->rawsize       = s;
    memblk->sequence      = memento.sequence;
    memblk->lastCheckedOK = memblk->sequence;
    memblk->flags         = 0;
    memblk->label         = 0;
    memblk->child         = MEMENTO_CHILD_MAGIC;
    memblk->sibling       = MEMENTO_SIBLING_MAGIC;
#ifdef MEMENTO_DETAILS
    memblk->details       = NULL;
    memblk->details_tail  = &memblk->details;
    Memento_storeDetails(memblk, Memento_EventType_malloc);
#endif /* MEMENTO_DETAILS */
    Memento_addBlockHead(&memento.used, memblk, 0);

    if (memento.leaking > 0)
        memblk->flags |= Memento_Flag_KnownLeak;

    return MEMBLK_TOBLK(memblk);
}

void *Memento_malloc(size_t s)
{
    void *ret;

    if (!memento.inited)
        Memento_init();

    MEMENTO_LOCK();
    ret = do_malloc(s, Memento_EventType_malloc);
    MEMENTO_UNLOCK();
    return ret;
}

void *Memento_calloc(size_t n, size_t s)
{
    void *block;

    if (!memento.inited)
        Memento_init();

    MEMENTO_LOCK();
    block = do_malloc(n*s, Memento_EventType_calloc);
    if (block)
        memset(block, 0, n*s);
    MEMENTO_UNLOCK();
    return block;
}

static void do_reference(Memento_BlkHeader *blk, int event)
;

int Memento_checkPointerOrNull(void *blk)
;

int Memento_checkBytePointerOrNull(void *blk)
;

int Memento_checkShortPointerOrNull(void *blk)
;

int Memento_checkIntPointerOrNull(void *blk)
;

static void *do_takeRef(void *blk)
{
    MEMENTO_LOCK();
    do_reference(safe_find_block(blk), Memento_EventType_takeRef);
    MEMENTO_UNLOCK();
    return blk;
}

void *Memento_takeByteRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    (void)Memento_checkBytePointerOrNull(blk);

    return do_takeRef(blk);
}

void *Memento_takeShortRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    (void)Memento_checkShortPointerOrNull(blk);

    return do_takeRef(blk);
}

void *Memento_takeIntRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    (void)Memento_checkIntPointerOrNull(blk);

    return do_takeRef(blk);
}

void *Memento_takeRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    return do_takeRef(blk);
}

static void *do_dropRef(void *blk)
{
    MEMENTO_LOCK();
    do_reference(safe_find_block(blk), Memento_EventType_dropRef);
    MEMENTO_UNLOCK();
    return blk;
}

void *Memento_dropByteRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    Memento_checkBytePointerOrNull(blk);

    return do_dropRef(blk);
}

void *Memento_dropShortRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    Memento_checkShortPointerOrNull(blk);

    return do_dropRef(blk);
}

void *Memento_dropIntRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    Memento_checkIntPointerOrNull(blk);

    return do_dropRef(blk);
}

void *Memento_dropRef(void *blk)
{
    if (!memento.inited)
        Memento_init();

    if (Memento_event()) Memento_breakpoint();

    if (!blk)
        return NULL;

    return do_dropRef(blk);
}

void *Memento_adjustRef(void *blk, int adjust)
{
    if (Memento_event()) Memento_breakpoint();

    if (blk == NULL)
        return NULL;

    while (adjust > 0)
    {
        do_takeRef(blk);
        adjust--;
    }
    while (adjust < 0)
    {
        do_dropRef(blk);
        adjust++;
    }

    return blk;
 }

void *Memento_reference(void *blk)
{
    if (!blk)
        return NULL;

    if (!memento.inited)
        Memento_init();

    MEMENTO_LOCK();
    do_reference(safe_find_block(blk), Memento_EventType_reference);
    MEMENTO_UNLOCK();
    return blk;
}

/* Treat blocks from the user with suspicion, and check them the slow
 * but safe way. */
static int checkBlockUser(Memento_BlkHeader *memblk, const char *action)
;

static int checkBlock(Memento_BlkHeader *memblk, const char *action)
;

static void do_free(void *blk, int eventType)
;

void Memento_free(void *blk)
{
    if (!memento.inited)
        Memento_init();

    MEMENTO_LOCK();
    do_free(blk, Memento_EventType_free);
    MEMENTO_UNLOCK();
}

static void *do_realloc(void *blk, size_t newsize, int type)
{
    Memento_BlkHeader *memblk, *newmemblk;
    size_t             newsizemem;
    int                flags;

    if (Memento_failThisEventLocked())
        return NULL;

    memblk     = MEMBLK_FROMBLK(blk);
    VALGRIND_MAKE_MEM_DEFINED(memblk, sizeof(*memblk));
    if (checkBlock(memblk, "realloc"))
        return NULL;

#ifdef MEMENTO_DETAILS
    Memento_storeDetails(memblk, type);
#endif

    VALGRIND_MAKE_MEM_DEFINED(memblk, sizeof(*memblk));
    if (memblk->flags & Memento_Flag_BreakOnRealloc)
        Memento_breakpointLocked();

    VALGRIND_MAKE_MEM_DEFINED(memblk, sizeof(*memblk));
    if (memento.maxMemory != 0 && memento.alloc - memblk->rawsize + newsize > memento.maxMemory)
        return NULL;

    newsizemem = MEMBLK_SIZE(newsize);
    Memento_removeBlock(&memento.used, memblk);
    VALGRIND_MAKE_MEM_DEFINED(memblk, sizeof(*memblk));
    flags = memblk->flags;
    newmemblk  = MEMENTO_UNDERLYING_REALLOC(memblk, newsizemem);
    if (newmemblk == NULL)
    {
        Memento_addBlockHead(&memento.used, memblk, 2);
        return NULL;
    }
    memento.numReallocs++;
    memento.totalAlloc += newsize;
    memento.alloc      -= newmemblk->rawsize;
    memento.alloc      += newsize;
    if (memento.peakAlloc < memento.alloc)
        memento.peakAlloc = memento.alloc;
    newmemblk->flags = flags;
#ifndef MEMENTO_LEAKONLY
    if (newmemblk->rawsize < newsize) {
        char *newbytes = ((char *)MEMBLK_TOBLK(newmemblk))+newmemblk->rawsize;
        VALGRIND_MAKE_MEM_DEFINED(newbytes, newsize - newmemblk->rawsize);
        memset(newbytes, MEMENTO_ALLOCFILL, newsize - newmemblk->rawsize);
        VALGRIND_MAKE_MEM_UNDEFINED(newbytes, newsize - newmemblk->rawsize);
    }
#endif
    newmemblk->rawsize = newsize;
#ifndef MEMENTO_LEAKONLY
    VALGRIND_MAKE_MEM_DEFINED(newmemblk->preblk, Memento_PreSize);
    memset(newmemblk->preblk, MEMENTO_PREFILL, Memento_PreSize);
    VALGRIND_MAKE_MEM_UNDEFINED(newmemblk->preblk, Memento_PreSize);
    VALGRIND_MAKE_MEM_DEFINED(MEMBLK_POSTPTR(newmemblk), Memento_PostSize);
    memset(MEMBLK_POSTPTR(newmemblk), MEMENTO_POSTFILL, Memento_PostSize);
    VALGRIND_MAKE_MEM_UNDEFINED(MEMBLK_POSTPTR(newmemblk), Memento_PostSize);
#endif
    Memento_addBlockHead(&memento.used, newmemblk, 2);
    return MEMBLK_TOBLK(newmemblk);
}

void *Memento_realloc(void *blk, size_t newsize)
{
    void *ret;

    if (!memento.inited)
        Memento_init();

    if (blk == NULL)
    {
        MEMENTO_LOCK();
        ret = do_malloc(newsize, Memento_EventType_realloc);
        MEMENTO_UNLOCK();
        return ret;
    }
    if (newsize == 0) {
        MEMENTO_LOCK();
        do_free(blk, Memento_EventType_realloc);
        MEMENTO_UNLOCK();
        return NULL;
    }

    MEMENTO_LOCK();
    ret = do_realloc(blk, newsize, Memento_EventType_realloc);
    MEMENTO_UNLOCK();
    return ret;
}

int Memento_checkBlock(void *blk)
;

#ifndef MEMENTO_LEAKONLY
static int Memento_Internal_checkAllAlloced(Memento_BlkHeader *memblk, void *arg)
;

static int Memento_Internal_checkAllFreed(Memento_BlkHeader *memblk, void *arg)
;
#endif /* MEMENTO_LEAKONLY */

static int Memento_checkAllMemoryLocked(void)
;

int Memento_checkAllMemory(void)
{
#ifndef MEMENTO_LEAKONLY
    int ret;

    MEMENTO_LOCK();
    ret = Memento_checkAllMemoryLocked();
    MEMENTO_UNLOCK();
    if (ret & 6) {
        Memento_breakpoint();
        return 1;
    }
    return 0;
#endif
}

int Memento_setParanoia(int i)
{
    memento.paranoia = i;
    if (memento.paranoia > 0)
        memento.countdown = memento.paranoia;
    else
        memento.countdown = -memento.paranoia;
    return i;
}

int Memento_paranoidAt(int i)
{
    memento.paranoidAt = i;
    return i;
}

int Memento_getBlockNum(void *b)
;

int Memento_check(void)
{
    int result;

    fprintf(stderr, "Checking memory\n");
    result = Memento_checkAllMemory();
    fprintf(stderr, "Memory checked!\n");
    return result;
}

int Memento_find(void *a)
;

void Memento_breakOnFree(void *a)
;

void Memento_breakOnRealloc(void *a)
;

int Memento_failAt(int i)
{
    memento.failAt = i;
    if ((memento.sequence > memento.failAt) &&
        (memento.failing != 0))
        Memento_startFailing();
    return i;
}

size_t Memento_setMax(size_t max)
{
    memento.maxMemory = max;
    return max;
}

void Memento_startLeaking(void)
{
    memento.leaking++;
}

void Memento_stopLeaking(void)
{
    memento.leaking--;
}

int Memento_squeezing(void)
{
    return memento.squeezing;
}

#endif /* MEMENTO_CPP_EXTRAS_ONLY */

#ifdef __cplusplus
/* Dumb overrides for the new and delete operators */

void *operator new(size_t size)
;

void  operator delete(void *pointer)
;

/* Some C++ systems (apparently) don't provide new[] or delete[]
 * operators. Provide a way to cope with this */
#ifndef MEMENTO_CPP_NO_ARRAY_CONSTRUCTORS
void *operator new[](size_t size)
{
    void *ret;
    if (!memento.inited)
        Memento_init();

    if (size == 0)
        size = 1;
    MEMENTO_LOCK();
    ret = do_malloc(size, Memento_EventType_newArray);
    MEMENTO_UNLOCK();
    return ret;
}

void  operator delete[](void *pointer)
{
    MEMENTO_LOCK();
    do_free(pointer, Memento_EventType_deleteArray);
    MEMENTO_UNLOCK();
}
#endif /* MEMENTO_CPP_NO_ARRAY_CONSTRUCTORS */
#endif /* __cplusplus */

#else

/* Just in case anyone has left some debugging code in... */
void (Memento_breakpoint)(void)
{
}

int (Memento_checkBlock)(void *b)
{
    return 0;
}

int (Memento_checkAllMemory)(void)
{
    return 0;
}

int (Memento_check)(void)
{
    return 0;
}

int (Memento_setParanoia)(int i)
{
    return 0;
}

int (Memento_paranoidAt)(int i)
{
    return 0;
}

int (Memento_breakAt)(int i)
{
    return 0;
}

int  (Memento_getBlockNum)(void *i)
{
    return 0;
}

int (Memento_find)(void *a)
{
    return 0;
}

int (Memento_failAt)(int i)
{
    return 0;
}

void (Memento_breakOnFree)(void *a)
{
}

void (Memento_breakOnRealloc)(void *a)
{
}

void *(Memento_takeRef)(void *a)
{
    return a;
}

void *(Memento_dropRef)(void *a)
{
    return a;
}

void *(Memento_adjustRef)(void *a, int adjust)
{
    return a;
}

void *(Memento_reference)(void *a)
{
    return a;
}

#undef Memento_malloc
#undef Memento_free
#undef Memento_realloc
#undef Memento_calloc

void *Memento_malloc(size_t size)
{
    return MEMENTO_UNDERLYING_MALLOC(size);
}

void Memento_free(void *b)
;

void *Memento_realloc(void *b, size_t s)
{
    return MEMENTO_UNDERLYING_REALLOC(b, s);
}

void *Memento_calloc(size_t n, size_t s)
{
    return MEMENTO_UNDERLYING_CALLOC(n, s);
}

void (Memento_listBlocks)(void)
{
}

void (Memento_listNewBlocks)(void)
{
}

size_t (Memento_setMax)(size_t max)
{
    return 0;
}

void (Memento_stats)(void)
{
}

void *(Memento_label)(void *ptr, const char *label)
{
    return ptr;
}

void (Memento_info)(void *addr)
{
}

void (Memento_listBlockInfo)(void)
{
}

void (Memento_startLeaking)(void)
{
}

void (Memento_stopLeaking)(void)
{
}

int (Memento_squeezing)(void)
{
    return 0;
}

#endif
