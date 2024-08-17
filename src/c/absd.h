#ifndef	_absd_user_
#define	_absd_user_

/* Module absd */

#include <string.h>
#include <mach/ndr.h>
#include <mach/boolean.h>
#include <mach/kern_return.h>
#include <mach/notify.h>
#include <mach/mach_types.h>
#include <mach/message.h>
#include <mach/mig_errors.h>
#include <mach/port.h>
	
/* BEGIN VOUCHER CODE */

#ifndef KERNEL
#if defined(__has_include)
#if __has_include(<mach/mig_voucher_support.h>)
#ifndef USING_VOUCHERS
#define USING_VOUCHERS
#endif
#ifndef __VOUCHER_FORWARD_TYPE_DECLS__
#define __VOUCHER_FORWARD_TYPE_DECLS__
#ifdef __cplusplus
extern "C" {
#endif
#ifndef __VOUCHER_FOWARD_TYPE_DECLS_SINGLE_ATTR
#define __VOUCHER_FOWARD_TYPE_DECLS_SINGLE_ATTR __unsafe_indexable
#endif
	extern boolean_t voucher_mach_msg_set(mach_msg_header_t * msg) __attribute__((weak_import));
#ifdef __cplusplus
}
#endif
#endif // __VOUCHER_FORWARD_TYPE_DECLS__
#endif // __has_include(<mach/mach_voucher_types.h>)
#endif // __has_include
#endif // !KERNEL
	
/* END VOUCHER CODE */

	
/* BEGIN MIG_STRNCPY_ZEROFILL CODE */

#if defined(__has_include)
#if __has_include(<mach/mig_strncpy_zerofill_support.h>)
#ifndef USING_MIG_STRNCPY_ZEROFILL
#define USING_MIG_STRNCPY_ZEROFILL
#endif
#ifndef __MIG_STRNCPY_ZEROFILL_FORWARD_TYPE_DECLS__
#define __MIG_STRNCPY_ZEROFILL_FORWARD_TYPE_DECLS__
#ifdef __cplusplus
extern "C" {
#endif
#ifndef __MIG_STRNCPY_ZEROFILL_FORWARD_TYPE_DECLS_CSTRING_ATTR
#define __MIG_STRNCPY_ZEROFILL_FORWARD_TYPE_DECLS_CSTRING_COUNTEDBY_ATTR(C) __unsafe_indexable
#endif
	extern int mig_strncpy_zerofill(char * dest, const char * src, int len) __attribute__((weak_import));
#ifdef __cplusplus
}
#endif
#endif /* __MIG_STRNCPY_ZEROFILL_FORWARD_TYPE_DECLS__ */
#endif /* __has_include(<mach/mig_strncpy_zerofill_support.h>) */
#endif /* __has_include */
	
/* END MIG_STRNCPY_ZEROFILL CODE */


#ifdef AUTOTEST
#ifndef FUNCTION_PTR_T
#define FUNCTION_PTR_T
typedef void (*function_ptr_t)(mach_port_t, char *, mach_msg_type_number_t);
typedef struct {
        char            * name;
        function_ptr_t  function;
} function_table_entry;
typedef function_table_entry   *function_table_t;
#endif /* FUNCTION_PTR_T */
#endif /* AUTOTEST */

#ifndef	absd_MSG_COUNT
#define	absd_MSG_COUNT	3
#endif	/* absd_MSG_COUNT */


#ifdef __BeforeMigUserHeader
__BeforeMigUserHeader
#endif /* __BeforeMigUserHeader */

#include <sys/cdefs.h>
__BEGIN_DECLS


/* Routine NACInit */
#ifdef	mig_external
mig_external
#else
extern
#endif	/* mig_external */
kern_return_t rawNACInit
(
	mach_port_t server,
	uint32_t magic,
	vm_offset_t cert,
	mach_msg_type_number_t certCnt,
	uint64_t *context,
	vm_offset_t *session_request,
	mach_msg_type_number_t *session_requestCnt
);

/* Routine NACKeyEstablishment */
#ifdef	mig_external
mig_external
#else
extern
#endif	/* mig_external */
kern_return_t rawNACKeyEstablishment
(
	mach_port_t server,
	uint32_t magic,
	uint64_t context,
	vm_offset_t session_response,
	mach_msg_type_number_t session_responseCnt
);

/* Routine NACSign */
#ifdef	mig_external
mig_external
#else
extern
#endif	/* mig_external */
kern_return_t rawNACSign
(
	mach_port_t server,
	uint32_t magic,
	uint64_t context,
	vm_offset_t data,
	mach_msg_type_number_t dataCnt,
	vm_offset_t *signature,
	mach_msg_type_number_t *signatureCnt
);

__END_DECLS

/********************** Caution **************************/
/* The following data types should be used to calculate  */
/* maximum message sizes only. The actual message may be */
/* smaller, and the position of the arguments within the */
/* message layout may vary from what is presented here.  */
/* For example, if any of the arguments are variable-    */
/* sized, and less than the maximum is sent, the data    */
/* will be packed tight in the actual message to reduce  */
/* the presence of holes.                                */
/********************** Caution **************************/

/* typedefs for all requests */

#ifndef __Request__absd_subsystem__defined
#define __Request__absd_subsystem__defined

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		/* start of the kernel processed data */
		mach_msg_body_t msgh_body;
		mach_msg_ool_descriptor_t cert;
		/* end of the kernel processed data */
		NDR_record_t NDR;
		uint32_t magic;
		mach_msg_type_number_t certCnt;
	} __Request__NACInit_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		/* start of the kernel processed data */
		mach_msg_body_t msgh_body;
		mach_msg_ool_descriptor_t session_response;
		/* end of the kernel processed data */
		NDR_record_t NDR;
		uint32_t magic;
		uint64_t context;
		mach_msg_type_number_t session_responseCnt;
	} __Request__NACKeyEstablishment_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		/* start of the kernel processed data */
		mach_msg_body_t msgh_body;
		mach_msg_ool_descriptor_t data;
		/* end of the kernel processed data */
		NDR_record_t NDR;
		uint32_t magic;
		uint64_t context;
		mach_msg_type_number_t dataCnt;
	} __Request__NACSign_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif
#endif /* !__Request__absd_subsystem__defined */

/* union of all requests */

#ifndef __RequestUnion__rawabsd_subsystem__defined
#define __RequestUnion__rawabsd_subsystem__defined
union __RequestUnion__rawabsd_subsystem {
	__Request__NACInit_t Request_rawNACInit;
	__Request__NACKeyEstablishment_t Request_rawNACKeyEstablishment;
	__Request__NACSign_t Request_rawNACSign;
};
#endif /* !__RequestUnion__rawabsd_subsystem__defined */
/* typedefs for all replies */

#ifndef __Reply__absd_subsystem__defined
#define __Reply__absd_subsystem__defined

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		/* start of the kernel processed data */
		mach_msg_body_t msgh_body;
		mach_msg_ool_descriptor_t session_request;
		/* end of the kernel processed data */
		NDR_record_t NDR;
		uint64_t context;
		mach_msg_type_number_t session_requestCnt;
	} __Reply__NACInit_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		NDR_record_t NDR;
		kern_return_t RetCode;
	} __Reply__NACKeyEstablishment_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif

#ifdef  __MigPackStructs
#pragma pack(push, 4)
#endif
	typedef struct {
		mach_msg_header_t Head;
		/* start of the kernel processed data */
		mach_msg_body_t msgh_body;
		mach_msg_ool_descriptor_t signature;
		/* end of the kernel processed data */
		NDR_record_t NDR;
		mach_msg_type_number_t signatureCnt;
	} __Reply__NACSign_t __attribute__((unused));
#ifdef  __MigPackStructs
#pragma pack(pop)
#endif
#endif /* !__Reply__absd_subsystem__defined */

/* union of all replies */

#ifndef __ReplyUnion__rawabsd_subsystem__defined
#define __ReplyUnion__rawabsd_subsystem__defined
union __ReplyUnion__rawabsd_subsystem {
	__Reply__NACInit_t Reply_rawNACInit;
	__Reply__NACKeyEstablishment_t Reply_rawNACKeyEstablishment;
	__Reply__NACSign_t Reply_rawNACSign;
};
#endif /* !__RequestUnion__rawabsd_subsystem__defined */

#ifndef subsystem_to_name_map_absd
#define subsystem_to_name_map_absd \
    { "NACInit", 1200 },\
    { "NACKeyEstablishment", 1201 },\
    { "NACSign", 1202 }
#endif

#ifdef __AfterMigUserHeader
__AfterMigUserHeader
#endif /* __AfterMigUserHeader */

#endif	 /* _absd_user_ */
