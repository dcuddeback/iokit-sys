use libc::{c_int,c_uint,clock_t};

pub type mach_port_t = c_uint;

// exports from <mach/kern_return.h> and <mach/i386/kern_return.h>

pub type kern_return_t = c_int;

pub const KERN_SUCCESS: kern_return_t = 0;

// exports from <mach/i386/boolean.h>

#[cfg(target_arch = "x86_64")]
pub type boolean_t = c_uint;

#[cfg(not(target_arch = "x86_64"))]
pub type boolean_t = c_int;

// exports from <mach/i386/vm_types.h>

pub type mach_vm_address_t = u64;
pub type mach_vm_offset_t  = u64;
pub type mach_vm_size_t    = u64;
pub type vm_map_offset_t   = u64;
pub type vm_map_address_t  = u64;
pub type vm_map_size_t     = u64;

pub type mach_port_context_t = mach_vm_address_t;

// exports from <mach/mach_types.h>

pub type task_t                  = mach_port_t;
pub type task_name_t             = mach_port_t;
pub type task_suspension_token_t = mach_port_t;
pub type thread_t                = mach_port_t;
pub type thread_act_t            = mach_port_t;
pub type ipc_space_t             = mach_port_t;
pub type coalition_t             = mach_port_t;
pub type host_t                  = mach_port_t;
pub type host_priv_t             = mach_port_t;
pub type host_security_t         = mach_port_t;
pub type processor_t             = mach_port_t;
pub type processor_set_t         = mach_port_t;
pub type processor_set_control_t = mach_port_t;
pub type semaphore_t             = mach_port_t;
pub type lock_set_t              = mach_port_t;
pub type ledger_t                = mach_port_t;
pub type alarm_t                 = mach_port_t;
pub type clock_serv_t            = mach_port_t;
pub type clock_ctrl_t            = mach_port_t;

pub type processor_set_name_t = processor_set_t;

pub type clock_reply_t             = mach_port_t;
pub type bootstrap_t               = mach_port_t;
pub type mem_entry_name_port_t     = mach_port_t;
pub type exception_handler_t       = mach_port_t;
pub type exception_handler_array_t = *mut exception_handler_t;
pub type vm_task_entry_t           = mach_port_t;
pub type io_master_t               = mach_port_t;
pub type UNDServerRef              = mach_port_t;

pub type task_array_t               = *mut task_t;
pub type thread_array_t             = *mut thread_t;
pub type processor_set_array_t      = *mut processor_set_t;
pub type processor_set_name_array_t = *mut processor_set_t;
pub type processor_array_t          = *mut processor_t;
pub type thread_act_array_t         = *mut thread_act_t;
pub type ledger_array_t             = *mut ledger_t;

pub type task_port_t                     = task_t;
pub type task_port_array_t               = task_array_t;
pub type thread_port_t                   = thread_t;
pub type thread_port_array_t             = thread_array_t;
pub type ipc_space_port_t                = ipc_space_t;
pub type host_name_t                     = host_t;
pub type host_name_port_t                = host_t;
pub type processor_set_port_t            = processor_set_t;
pub type processor_set_name_port_t       = processor_set_t;
pub type processor_set_name_port_array_t = processor_set_array_t;
pub type processor_set_control_port_t    = processor_set_t;
pub type processor_port_t                = processor_t;
pub type processor_port_array_t          = processor_array_t;
pub type thread_act_port_t               = thread_act_t;
pub type thread_act_port_array_t         = thread_act_array_t;
pub type semaphore_port_t                = semaphore_t;
pub type lock_set_port_t                 = lock_set_t;
pub type ledger_port_t                   = ledger_t;
pub type ledger_port_array_t             = ledger_array_t;
pub type alarm_port_t                    = alarm_t;
pub type clock_serv_port_t               = clock_serv_t;
pub type clock_ctrl_port_t               = clock_ctrl_t;
pub type exception_port_t                = exception_handler_t;
pub type exception_port_arrary_t         = exception_handler_array_t;

pub const TASK_NULL:          task_t          = 0;
pub const TASK_NAME_NULL:     task_name_t     = 0;
pub const THREAD_NULL:        thread_t        = 0;
pub const TID_NULL:           u64             = 0;
pub const THR_ACT_NULL:       thread_act_t    = 0;
pub const IPC_SPACE_NULL:     ipc_space_t     = 0;
pub const COALITION_NULL:     coalition_t     = 0;
pub const HOST_NULL:          host_t          = 0;
pub const HOST_PRIV_NULL:     host_priv_t     = 0;
pub const HOST_SECURITY_NULL: host_security_t = 0;
pub const PROCESSOR_SET_NULL: processor_set_t = 0;
pub const PROCESSOR_NULL:     processor_t     = 0;
pub const SEMAPHORE_NULL:     semaphore_t     = 0;
pub const LOCK_SET_NULL:      lock_set_t      = 0;
pub const LEDGER_NULL:        ledger_t        = 0;
pub const ALARM_NULL:         alarm_t         = 0;
pub const CLOCK_NULL:         clock_t         = 0;
pub const UND_SERVER_NULL:    UNDServerRef    = 0;

// exports from <mach/clock_types.h>

pub type alarm_type_t = c_int;
pub type sleep_type_t = c_int;
pub type clock_id_t = c_int;
pub type clock_flavor_t = c_int;
pub type clock_attr_t = *mut c_int;
pub type clock_res_t = c_int;

#[repr(C)]
struct mach_timespec {
    tv_sec: c_uint,
    tv_nsec: clock_res_t
}
pub type mach_timespec_t = mach_timespec;
