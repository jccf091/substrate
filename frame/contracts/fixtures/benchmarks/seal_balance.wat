(module
	(import "env" "memory" (memory 1 1))
	(import "seal0" "seal_balance" (func $balance (param i32 i32)))

	;; size of the buffer where balance is copied
	(data (i32.const 0) "\FF\00\00\00")

	(func (export "deploy"))

	(func (export "call")
		(call $balance (i32.const 4) (i32.const 0))
	)
)
