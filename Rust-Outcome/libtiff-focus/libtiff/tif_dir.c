/* $Id: tif_dir.c,v 1.131 2017-07-11 21:38:04 erouault Exp $ */

/*
 * Copyright (c) 1988-1997 Sam Leffler
 * Copyright (c) 1991-1997 Silicon Graphics, Inc.
 *
 * Permission to use, copy, modify, distribute, and sell this software and 
 * its documentation for any purpose is hereby granted without fee, provided
 * that (i) the above copyright notices and this permission notice appear in
 * all copies of the software and related documentation, and (ii) the names of
 * Sam Leffler and Silicon Graphics may not be used in any advertising or
 * publicity relating to the software without the specific, prior written
 * permission of Sam Leffler and Silicon Graphics.
 * 
 * THE SOFTWARE IS PROVIDED "AS-IS" AND WITHOUT WARRANTY OF ANY KIND, 
 * EXPRESS, IMPLIED OR OTHERWISE, INCLUDING WITHOUT LIMITATION, ANY 
 * WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.  
 * 
 * IN NO EVENT SHALL SAM LEFFLER OR SILICON GRAPHICS BE LIABLE FOR
 * ANY SPECIAL, INCIDENTAL, INDIRECT OR CONSEQUENTIAL DAMAGES OF ANY KIND,
 * OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
 * WHETHER OR NOT ADVISED OF THE POSSIBILITY OF DAMAGE, AND ON ANY THEORY OF 
 * LIABILITY, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE 
 * OF THIS SOFTWARE.
 */

/*
 * TIFF Library.
 *
 * Directory Tag Get & Set Routines.
 * (and also some miscellaneous stuff)
 */
#include "tiffiop.h"
#include <float.h>

/*
 * These are used in the backwards compatibility code...
 */
#define DATATYPE_VOID		0       /* !untyped data */
#define DATATYPE_INT		1       /* !signed integer data */
#define DATATYPE_UINT		2       /* !unsigned integer data */
#define DATATYPE_IEEEFP		3       /* !IEEE floating point data */

static void
setByteArray(void** vpp, void* vp, size_t nmemb, size_t elem_size)
;
void _TIFFsetByteArray(void** vpp, void* vp, uint32 n)
    ;
void _TIFFsetString(char** cpp, char* cp)
    ;
void _TIFFsetNString(char** cpp, char* cp, uint32 n)
    ;
void _TIFFsetShortArray(uint16** wpp, uint16* wp, uint32 n)
    ;
void _TIFFsetLongArray(uint32** lpp, uint32* lp, uint32 n)
    ;
void _TIFFsetLong8Array(uint64** lpp, uint64* lp, uint32 n)
    ;
void _TIFFsetFloatArray(float** fpp, float* fp, uint32 n)
    ;
void _TIFFsetDoubleArray(double** dpp, double* dp, uint32 n)
    ;

static void
setDoubleArrayOneValue(double** vpp, double value, size_t nmemb)
;

/*
 * Install extra samples information.
 */
static int
setExtraSamples(TIFFDirectory* td, va_list ap, uint32* v)
;

/*
 * Confirm we have "samplesperpixel" ink names separated by \0.  Returns 
 * zero if the ink names are not as expected.
 */
static uint32
checkInkNamesString(TIFF* tif, uint32 slen, const char* s)
;

float TIFFClampDoubleToFloat( double val )
;

static int
_TIFFVSetField(TIFF* tif, uint32 tag, va_list ap)
;

/*
 * Return 1/0 according to whether or not
 * it is permissible to set the tag's value.
 * Note that we allow ImageLength to be changed
 * so that we can append and extend to images.
 * Any other tag may not be altered once writing
 * has commenced, unless its value has no effect
 * on the format of the data that is written.
 */
static int
OkToChangeTag(TIFF* tif, uint32 tag)
;

/*
 * Record the value of a field in the
 * internal directory structure.  The
 * field will be written to the file
 * when/if the directory structure is
 * updated.
 */
int
TIFFSetField(TIFF* tif, uint32 tag, ...)
;

/*
 * Clear the contents of the field in the internal structure.
 */
int
TIFFUnsetField(TIFF* tif, uint32 tag)
;

/*
 * Like TIFFSetField, but taking a varargs
 * parameter list.  This routine is useful
 * for building higher-level interfaces on
 * top of the library.
 */
int
TIFFVSetField(TIFF* tif, uint32 tag, va_list ap)
;

static int
_TIFFVGetField(TIFF* tif, uint32 tag, va_list ap)
;

/*
 * Return the value of a field in the
 * internal directory structure.
 */
int
TIFFGetField(TIFF* tif, uint32 tag, ...)
;

/*
 * Like TIFFGetField, but taking a varargs
 * parameter list.  This routine is useful
 * for building higher-level interfaces on
 * top of the library.
 */
int
TIFFVGetField(TIFF* tif, uint32 tag, va_list ap)
;

#define	CleanupField(member) ;

/*
 * Release storage associated with a directory.
 */
void
TIFFFreeDirectory(TIFF* tif)
{
	TIFFDirectory *td = &tif->tif_dir;
	int            i;

	_TIFFmemset(td->td_fieldsset, 0, FIELD_SETLONGS);
	CleanupField(td_sminsamplevalue);
	CleanupField(td_smaxsamplevalue);
	CleanupField(td_colormap[0]);
	CleanupField(td_colormap[1]);
	CleanupField(td_colormap[2]);
	CleanupField(td_sampleinfo);
	CleanupField(td_subifd);
	CleanupField(td_inknames);
	CleanupField(td_refblackwhite);
	CleanupField(td_transferfunction[0]);
	CleanupField(td_transferfunction[1]);
	CleanupField(td_transferfunction[2]);
	CleanupField(td_stripoffset);
	CleanupField(td_stripbytecount);
	TIFFClrFieldBit(tif, FIELD_YCBCRSUBSAMPLING);
	TIFFClrFieldBit(tif, FIELD_YCBCRPOSITIONING);

	/* Cleanup custom tag values */
	for( i = 0; i < td->td_customValueCount; i++ ) {
		if (td->td_customValues[i].value)
			_TIFFfree(td->td_customValues[i].value);
	}

	td->td_customValueCount = 0;
	CleanupField(td_customValues);

#if defined(DEFER_STRILE_LOAD)
        _TIFFmemset( &(td->td_stripoffset_entry), 0, sizeof(TIFFDirEntry));
        _TIFFmemset( &(td->td_stripbytecount_entry), 0, sizeof(TIFFDirEntry));
#endif        
}
#undef CleanupField

/*
 * Client Tag extension support (from Niles Ritter).
 */
static TIFFExtendProc _TIFFextender = (TIFFExtendProc) NULL;

TIFFExtendProc
TIFFSetTagExtender(TIFFExtendProc extender)
;

/*
 * Setup for a new directory.  Should we automatically call
 * TIFFWriteDirectory() if the current one is dirty?
 *
 * The newly created directory will not exist on the file till
 * TIFFWriteDirectory(), TIFFFlush() or TIFFClose() is called.
 */
int
TIFFCreateDirectory(TIFF* tif)
;

int
TIFFCreateCustomDirectory(TIFF* tif, const TIFFFieldArray* infoarray)
;

int
TIFFCreateEXIFDirectory(TIFF* tif)
;

/*
 * Setup a default directory structure.
 */
int
TIFFDefaultDirectory(TIFF* tif)
;

static int
TIFFAdvanceDirectory(TIFF* tif, uint64* nextdir, uint64* off)
;

/*
 * Count the number of directories in a file.
 */
uint16
TIFFNumberOfDirectories(TIFF* tif)
;

/*
 * Set the n-th directory as the current directory.
 * NB: Directories are numbered starting at 0.
 */
int
TIFFSetDirectory(TIFF* tif, uint16 dirn)
;

/*
 * Set the current directory to be the directory
 * located at the specified file offset.  This interface
 * is used mainly to access directories linked with
 * the SubIFD tag (e.g. thumbnail images).
 */
int
TIFFSetSubDirectory(TIFF* tif, uint64 diroff)
;

/*
 * Return file offset of the current directory.
 */
uint64
TIFFCurrentDirOffset(TIFF* tif)
;

/*
 * Return an indication of whether or not we are
 * at the last directory in the file.
 */
int
TIFFLastDirectory(TIFF* tif)
;

/*
 * Unlink the specified directory from the directory chain.
 */
int
TIFFUnlinkDirectory(TIFF* tif, uint16 dirn)
;

/* vim: set ts=8 sts=8 sw=8 noet: */
/*
 * Local Variables:
 * mode: c
 * c-basic-offset: 8
 * fill-column: 78
 * End:
 */
