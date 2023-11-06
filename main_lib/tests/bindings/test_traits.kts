import uniffi.main_lib.*
import uniffi.shared_lib.*

class TT: CommonTrait {
    override fun foo(): String {
        return "hello"
    }
}

useCommonTrait(TT());