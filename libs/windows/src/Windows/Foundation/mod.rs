#[cfg(feature = "Foundation_Collections")]
pub mod Collections;
//#[cfg(feature = "Foundation_Diagnostics")]
//pub mod Diagnostics;
//#[cfg(feature = "Foundation_Metadata")]
//pub mod Metadata;\
//#[cfg(feature = "Foundation_Numerics")]
//pub mod Numerics;
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IAsyncAction(::windows::core::IUnknown);
impl IAsyncAction {
    pub fn SetCompleted(&self, handler: &AsyncActionCompletedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Completed(&self) -> ::windows::core::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IAsyncAction, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IAsyncAction> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncAction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAsyncAction> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncAction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IAsyncAction> for ::windows::core::InParam<IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncAction) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IAsyncAction {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncAction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncAction {}
impl ::core::fmt::Debug for IAsyncAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAsyncAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5a648006-843a-4da9-865b-9d26e5dfad7b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
impl IAsyncAction {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl ::std::future::Future for IAsyncAction {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
unsafe impl ::core::marker::Send for IAsyncAction {}
unsafe impl ::core::marker::Sync for IAsyncAction {}
unsafe impl ::windows::core::Vtable for IAsyncAction {
    type Vtable = IAsyncAction_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncAction {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncAction_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IAsyncActionWithProgress<TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    pub fn SetProgress(&self, handler: &AsyncActionProgressHandler<TProgress>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProgress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Progress(&self) -> ::windows::core::Result<AsyncActionProgressHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompleted(&self, handler: &AsyncActionWithProgressCompletedHandler<TProgress>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Completed(&self) -> ::windows::core::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncActionWithProgress<TProgress>> for ::windows::core::IUnknown {
    fn from(value: IAsyncActionWithProgress<TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncActionWithProgress<TProgress>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAsyncActionWithProgress<TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncActionWithProgress<TProgress>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncActionWithProgress<TProgress>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncActionWithProgress<TProgress>> for ::windows::core::IInspectable {
    fn from(value: IAsyncActionWithProgress<TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncActionWithProgress<TProgress>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAsyncActionWithProgress<TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncActionWithProgress<TProgress>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncActionWithProgress<TProgress>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncActionWithProgress<TProgress>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncActionWithProgress<TProgress>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncActionWithProgress<TProgress>> for ::windows::core::InParam<IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncActionWithProgress<TProgress>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IAsyncActionWithProgress<TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncActionWithProgress<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncActionWithProgress<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncActionWithProgress<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncActionWithProgress").field(&self.0).finish()
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncActionWithProgress<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{1f6db258-e803-48a1-9546-eb7353398884}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncActionWithProgress<TProgress> {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for IAsyncActionWithProgress<TProgress> {
    type Vtable = IAsyncActionWithProgress_Vtbl<TProgress>;
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncActionWithProgress<TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncActionWithProgress_Vtbl<TProgress>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IAsyncInfo(::windows::core::IUnknown);
impl IAsyncInfo {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IAsyncInfo, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IAsyncInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsyncInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsyncInfo {}
impl ::core::fmt::Debug for IAsyncInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAsyncInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IAsyncInfo {
    type Vtable = IAsyncInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAsyncInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000036_0000_0000_c000_000000000046);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncInfo_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut AsyncStatus) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IAsyncOperation<TResult>(::windows::core::IUnknown, ::core::marker::PhantomData<TResult>)
where
    TResult: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn SetCompleted(&self, handler: &AsyncOperationCompletedHandler<TResult>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Completed(&self) -> ::windows::core::Result<AsyncOperationCompletedHandler<TResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows::core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperation<TResult>> for ::windows::core::IUnknown {
    fn from(value: IAsyncOperation<TResult>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncOperation<TResult>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAsyncOperation<TResult>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperation<TResult>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncOperation<TResult>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperation<TResult>> for ::windows::core::IInspectable {
    fn from(value: IAsyncOperation<TResult>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncOperation<TResult>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAsyncOperation<TResult>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperation<TResult>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncOperation<TResult>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncOperation<TResult>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperation<TResult>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperation<TResult>> for ::windows::core::InParam<IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperation<TResult>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IAsyncOperation<TResult> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TResult>)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncOperation<TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncOperation<TResult> {}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncOperation<TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncOperation").field(&self.0).finish()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncOperation<TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9fc2b0bb-e446-44e2-aa61-9cab8f636af2}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn get(&self) -> ::windows::core::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncOperation<TResult> {
    type Output = ::windows::core::Result<TResult>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncOperation<TResult> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for IAsyncOperation<TResult> {
    type Vtable = IAsyncOperation_Vtbl<TResult>;
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncOperation<TResult> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperation_Vtbl<TResult>
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgress<TResult, TProgress> {
    pub fn SetProgress(&self, handler: &AsyncOperationProgressHandler<TResult, TProgress>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetProgress)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Progress(&self) -> ::windows::core::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Progress)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetCompleted(&self, handler: &AsyncOperationWithProgressCompletedHandler<TResult, TProgress>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetCompleted)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    pub fn Completed(&self) -> ::windows::core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Completed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetResults(&self) -> ::windows::core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetResults)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Id)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Status)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ErrorCode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Cancel)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IUnknown {
    fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncOperationWithProgress<TResult, TProgress>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IInspectable {
    fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IAsyncOperationWithProgress<TResult, TProgress>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::InParam<IAsyncInfo> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IAsyncOperationWithProgress<TResult, TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TResult>, ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IAsyncOperationWithProgress<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IAsyncOperationWithProgress<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IAsyncOperationWithProgress<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsyncOperationWithProgress").field(&self.0).finish()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{b5d036d7-e297-498f-ba60-0289e76e23dd}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgress<TResult, TProgress> {
    pub fn get(&self) -> ::windows::core::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::core::Waiter::new()?;
            self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncOperationWithProgress<TResult, TProgress> {
    type Output = ::windows::core::Result<TResult>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for IAsyncOperationWithProgress<TResult, TProgress> {
    type Vtable = IAsyncOperationWithProgress_Vtbl<TResult, TProgress>;
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncOperationWithProgress<TResult, TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperationWithProgress_Vtbl<TResult, TProgress>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Progress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResults: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IClosable(::windows::core::IUnknown);
impl IClosable {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IClosable, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IClosable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClosable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClosable {}
impl ::core::fmt::Debug for IClosable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClosable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IClosable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30d5a829-7fa4-4026-83bb-d75bae4ea99e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IClosable {
    type Vtable = IClosable_Vtbl;
}
unsafe impl ::windows::core::Interface for IClosable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosable_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeferral {
    type Vtable = IDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeferral {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6269732_3b7f_46a7_b40b_4fdca2a2c693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferral_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeferralFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IDeferralFactory {
    type Vtable = IDeferralFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IDeferralFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferralFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IGetActivationFactory(::windows::core::IUnknown);
impl IGetActivationFactory {
    pub fn GetActivationFactory(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetActivationFactory)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(activatableclassid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IGetActivationFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IGetActivationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGetActivationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetActivationFactory {}
impl ::core::fmt::Debug for IGetActivationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetActivationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGetActivationFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4edb8ee2-96dd-49a7-94f7-4607ddab8e3c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IGetActivationFactory {
    type Vtable = IGetActivationFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IGetActivationFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4edb8ee2_96dd_49a7_94f7_4607ddab8e3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetActivationFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetActivationFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, activatableclassid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGuidHelperStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IGuidHelperStatics {
    type Vtable = IGuidHelperStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IGuidHelperStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59c7966b_ae52_5283_ad7f_a1b9e9678add);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidHelperStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateNewGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Empty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: &::windows::core::GUID, value: &::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IMemoryBuffer(::windows::core::IUnknown);
impl IMemoryBuffer {
    pub fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateReference)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IMemoryBuffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IMemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMemoryBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBuffer> for ::windows::core::InParam<IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBuffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IMemoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMemoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBuffer {}
impl ::core::fmt::Debug for IMemoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMemoryBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc4dd2a-245b-11e4-af98-689423260cf8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMemoryBuffer {
    type Vtable = IMemoryBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd2a_245b_11e4_af98_689423260cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBuffer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMemoryBufferFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IMemoryBufferFactory {
    type Vtable = IMemoryBufferFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryBufferFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd2b_245b_11e4_af98_689423260cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IMemoryBufferReference(::windows::core::IUnknown);
impl IMemoryBufferReference {
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Capacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Closed(&self, handler: &TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>) -> ::windows::core::Result<EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Closed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RemoveClosed(&self, cookie: EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveClosed)(::windows::core::Vtable::as_raw(this), cookie).ok() }
    }
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
::windows::core::interface_hierarchy!(IMemoryBufferReference, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IMemoryBufferReference> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMemoryBufferReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBufferReference> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBufferReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBufferReference> for ::windows::core::InParam<IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBufferReference) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IMemoryBufferReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMemoryBufferReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMemoryBufferReference {}
impl ::core::fmt::Debug for IMemoryBufferReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMemoryBufferReference").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMemoryBufferReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc4dd29-245b-11e4-af98-689423260cf8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IMemoryBufferReference {
    type Vtable = IMemoryBufferReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IMemoryBufferReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd29_245b_11e4_af98_689423260cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferReference_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut EventRegistrationToken) -> ::windows::core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: EventRegistrationToken) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IPropertyValue(::windows::core::IUnknown);
impl IPropertyValue {
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNumericScalar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt8)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSingle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDouble)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChar16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt8Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetSingleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetDoubleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetChar16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetBooleanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetStringArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetInspectableArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetGuidArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetDateTimeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetTimeSpanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetPointArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetSizeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).GetRectArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
::windows::core::interface_hierarchy!(IPropertyValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IPropertyValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPropertyValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertyValue {}
impl ::core::fmt::Debug for IPropertyValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertyValue").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertyValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IPropertyValue {
    type Vtable = IPropertyValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd682dd_7554_40e9_9a9b_82654ede7e62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows::core::HRESULT,
    pub IsNumericScalar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT,
    pub GetInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT,
    pub GetUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT,
    pub GetUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub GetSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub GetChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows::core::HRESULT,
    pub GetTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows::core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows::core::HRESULT,
    pub GetRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows::core::HRESULT,
    pub GetUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT,
    pub GetUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub GetInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT,
    pub GetUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT,
    pub GetInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT,
    pub GetUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT,
    pub GetSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT,
    pub GetDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT,
    pub GetChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetInspectableArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT,
    pub GetPointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT,
    pub GetSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT,
    pub GetRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyValueStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IPropertyValueStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSingle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDouble: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChar16: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInspectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTimeSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt8Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt32Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateUInt64Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSingleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDoubleArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateChar16Array: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBooleanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateStringArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateInspectableArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGuidArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDateTimeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateTimeSpanArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePointArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSizeArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRectArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IReference<T>(::windows::core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
impl<T: ::windows::core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> ::windows::core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNumericScalar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt8)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSingle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDouble)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChar16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt8Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetSingleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetDoubleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetChar16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetBooleanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetStringArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInspectableArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetGuidArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetDateTimeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetTimeSpanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetPointArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetSizeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetRectArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IUnknown {
    fn from(value: IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReference<T>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IUnknown {
    fn from(value: &IReference<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IInspectable {
    fn from(value: IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReference<T>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IReference<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IInspectable {
    fn from(value: &IReference<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: IReference<T>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReference<T>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for ::windows::core::InParam<IPropertyValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReference<T>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IReference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IReference<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IReference<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IReference<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReference").field(&self.0).finish()
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IReference<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IReference<T> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_Vtbl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IReferenceArray<T>(::windows::core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
impl<T: ::windows::core::RuntimeType + 'static> IReferenceArray<T> {
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::Array<T>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<T>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsNumericScalar)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt8)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt32)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetUInt64)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSingle)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDouble)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetChar16)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetBoolean)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDateTime)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetTimeSpan)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPoint)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSize)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetRect)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt8Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt32Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetUInt64Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetSingleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetDoubleArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetChar16Array)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetBooleanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetStringArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetInspectableArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetGuidArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetDateTimeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetTimeSpanArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetPointArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetSizeArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).GetRectArray)(::windows::core::Vtable::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReferenceArray<T>> for ::windows::core::IUnknown {
    fn from(value: IReferenceArray<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReferenceArray<T>> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReferenceArray<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReferenceArray<T>> for ::windows::core::IUnknown {
    fn from(value: &IReferenceArray<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReferenceArray<T>> for ::windows::core::IInspectable {
    fn from(value: IReferenceArray<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&'a IReferenceArray<T>> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IReferenceArray<T>) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReferenceArray<T>> for ::windows::core::IInspectable {
    fn from(value: &IReferenceArray<T>) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: IReferenceArray<T>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReferenceArray<T>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReferenceArray<T>> for ::windows::core::InParam<IPropertyValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReferenceArray<T>) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for IReferenceArray<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for IReferenceArray<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for IReferenceArray<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for IReferenceArray<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceArray").field(&self.0).finish()
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IReferenceArray<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17707-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for IReferenceArray<T> {
    type Vtable = IReferenceArray_Vtbl<T>;
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IReferenceArray<T> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceArray_Vtbl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IStringable(::windows::core::IUnknown);
impl IStringable {
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IStringable, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IStringable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStringable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringable {}
impl ::core::fmt::Debug for IStringable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStringable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IStringable {
    type Vtable = IStringable_Vtbl;
}
unsafe impl ::windows::core::Interface for IStringable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUriEscapeStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUriEscapeStatics {
    type Vtable = IUriEscapeStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriEscapeStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub UnescapeComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tounescape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, toescape: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUriRuntimeClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUriRuntimeClass {
    type Vtable = IUriRuntimeClass_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriRuntimeClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e365e57_48b2_4160_956f_c7385120bbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AbsoluteUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Extension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Fragment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Host: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Password: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryParsed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RawUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UserName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Port: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Suspicious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Equals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUriRuntimeClassFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUriRuntimeClassFactory {
    type Vtable = IUriRuntimeClassFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriRuntimeClassFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44a9796f_723e_4fdf_a218_033e75b0c084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseuri: *mut ::core::ffi::c_void, relativeuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUriRuntimeClassWithAbsoluteCanonicalUri {
    type Vtable = IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl;
}
unsafe impl ::windows::core::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x758d9661_221c_480f_a339_50656673f46f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct IWwwFormUrlDecoderEntry(::windows::core::IUnknown);
impl IWwwFormUrlDecoderEntry {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IWwwFormUrlDecoderEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWwwFormUrlDecoderEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWwwFormUrlDecoderEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWwwFormUrlDecoderEntry {}
impl ::core::fmt::Debug for IWwwFormUrlDecoderEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWwwFormUrlDecoderEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{125e7431-f678-4e8e-b670-20a9b06c512d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderEntry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x125e7431_f678_4e8e_b670_20a9b06c512d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderEntry_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClass(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWwwFormUrlDecoderRuntimeClass {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_Vtbl;
}
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderRuntimeClass {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45a0451_f225_4542_9296_0e1df5d254df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetFirstValueByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWwwFormUrlDecoderRuntimeClassFactory {
    type Vtable = IWwwFormUrlDecoderRuntimeClassFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderRuntimeClassFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct Deferral(::windows::core::IUnknown);
impl Deferral {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Complete)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Create(handler: &DeferralCompletedHandler) -> ::windows::core::Result<Deferral> {
        Self::IDeferralFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Deferral, IDeferralFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Deferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Deferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Deferral {}
impl ::core::fmt::Debug for Deferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Deferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Deferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Deferral;{d6269732-3b7f-46a7-b40b-4fdca2a2c693})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Deferral {
    type Vtable = IDeferral_Vtbl;
}
unsafe impl ::windows::core::Interface for Deferral {
    const IID: ::windows::core::GUID = <IDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
::windows::core::interface_hierarchy!(Deferral, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Deferral> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Deferral) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Deferral> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Deferral) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&Deferral> for ::windows::core::InParam<IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Deferral) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Deferral {}
unsafe impl ::core::marker::Sync for Deferral {}
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct GuidHelper;
impl GuidHelper {
    pub fn CreateNewGuid() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateNewGuid)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Empty() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Empty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Equals(target: ::windows::core::GUID, value: ::windows::core::GUID) -> ::windows::core::Result<bool> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Equals)(::windows::core::Vtable::as_raw(this), &target, &value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IGuidHelperStatics<R, F: FnOnce(&IGuidHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<GuidHelper, IGuidHelperStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct MemoryBuffer(::windows::core::IUnknown);
impl MemoryBuffer {
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateReference)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(capacity: u32) -> ::windows::core::Result<MemoryBuffer> {
        Self::IMemoryBufferFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), capacity, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMemoryBufferFactory<R, F: FnOnce(&IMemoryBufferFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<MemoryBuffer, IMemoryBufferFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for MemoryBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MemoryBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MemoryBuffer {}
impl ::core::fmt::Debug for MemoryBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MemoryBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MemoryBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.MemoryBuffer;{fbc4dd2a-245b-11e4-af98-689423260cf8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for MemoryBuffer {
    type Vtable = IMemoryBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for MemoryBuffer {
    const IID: ::windows::core::GUID = <IMemoryBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.MemoryBuffer";
}
::windows::core::interface_hierarchy!(MemoryBuffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<MemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MemoryBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MemoryBuffer> for ::windows::core::InParam<IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<MemoryBuffer> for IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: MemoryBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MemoryBuffer> for IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&MemoryBuffer> for ::windows::core::InParam<IMemoryBuffer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for MemoryBuffer {}
unsafe impl ::core::marker::Sync for MemoryBuffer {}
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct PropertyValue;
impl PropertyValue {
    pub fn CreateEmpty() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateEmpty)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt8(value: u8) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt8)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt16(value: i16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt16)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt16)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt32(value: i32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt32)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt32(value: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt32)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt64(value: i64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt64)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt64(value: u64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt64)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSingle(value: f32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSingle)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateDouble(value: f64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDouble)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateChar16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateChar16)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBoolean(value: bool) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBoolean)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateString(value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateString)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInspectable<P0>(value: P0) -> ::windows::core::Result<::windows::core::IInspectable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInspectable)(::windows::core::Vtable::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateGuid(value: ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateGuid)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateDateTime(value: DateTime) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDateTime)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTimeSpan(value: TimeSpan) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTimeSpan)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreatePoint(value: Point) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePoint)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSize(value: Size) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSize)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateRect(value: Rect) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRect)(::windows::core::Vtable::as_raw(this), value, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt8Array(value: &[u8]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt8Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt16Array(value: &[i16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt16Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt16Array(value: &[u16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt16Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt32Array(value: &[i32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt32Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt32Array(value: &[u32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt32Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInt64Array(value: &[i64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInt64Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateUInt64Array(value: &[u64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUInt64Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSingleArray(value: &[f32]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSingleArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateDoubleArray(value: &[f64]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDoubleArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateChar16Array(value: &[u16]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateChar16Array)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateBooleanArray(value: &[bool]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateBooleanArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateStringArray(value: &[::windows::core::HSTRING]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateStringArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateInspectableArray(value: &[::core::option::Option<::windows::core::IInspectable>]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateInspectableArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateGuidArray(value: &[::windows::core::GUID]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateGuidArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateDateTimeArray(value: &[DateTime]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateDateTimeArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateTimeSpanArray(value: &[TimeSpan]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateTimeSpanArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreatePointArray(value: &[Point]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreatePointArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateSizeArray(value: &[Size]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateSizeArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateRectArray(value: &[Rect]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateRectArray)(::windows::core::Vtable::as_raw(this), value.len() as u32, value.as_ptr(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PropertyValue, IPropertyValueStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct Uri(::windows::core::IUnknown);
impl Uri {
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ToString)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UnescapeComponent(tounescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UnescapeComponent)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(tounescape), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn EscapeComponent(toescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EscapeComponent)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(toescape), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AbsoluteUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AbsoluteUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Domain)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Extension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Extension)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Fragment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Fragment)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Host)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Password)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Path)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Query(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Query)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn QueryParsed(&self) -> ::windows::core::Result<WwwFormUrlDecoder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).QueryParsed)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn RawUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RawUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SchemeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SchemeName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UserName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Port(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Port)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Suspicious(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Suspicious)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Equals(&self, puri: &Uri) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Equals)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(puri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CombineUri(&self, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CombineUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(relativeuri), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateUri(uri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithRelativeUri(baseuri: &::windows::core::HSTRING, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithRelativeUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(baseuri), ::core::mem::transmute_copy(relativeuri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AbsoluteCanonicalUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AbsoluteCanonicalUri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DisplayIri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DisplayIri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Uri, IUriEscapeStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<Uri, IUriRuntimeClassFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for Uri {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Uri {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Uri {}
impl ::core::fmt::Debug for Uri {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Uri").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Uri {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for Uri {
    type Vtable = IUriRuntimeClass_Vtbl;
}
unsafe impl ::windows::core::Interface for Uri {
    const IID: ::windows::core::GUID = <IUriRuntimeClass as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
::windows::core::interface_hierarchy!(Uri, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<Uri> for IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: Uri) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Uri> for IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Uri) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&Uri> for ::windows::core::InParam<IStringable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &Uri) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for Uri {}
unsafe impl ::core::marker::Sync for Uri {}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct WwwFormUrlDecoder(::windows::core::IUnknown);
impl WwwFormUrlDecoder {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<Collections::IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &::windows::core::Interface::cast::<Collections::IIterable<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IWwwFormUrlDecoderEntry> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetAt)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0, E0>(&self, value: P0, index: &mut u32) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IWwwFormUrlDecoderEntry>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IndexOf)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi(), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<IWwwFormUrlDecoderEntry>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetMany)(::windows::core::Vtable::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetFirstValueByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFirstValueByName)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CreateWwwFormUrlDecoder(query: &::windows::core::HSTRING) -> ::windows::core::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWwwFormUrlDecoder)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(query), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWwwFormUrlDecoderRuntimeClassFactory<R, F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WwwFormUrlDecoder, IWwwFormUrlDecoderRuntimeClassFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WwwFormUrlDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WwwFormUrlDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WwwFormUrlDecoder {}
impl ::core::fmt::Debug for WwwFormUrlDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwwFormUrlDecoder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WwwFormUrlDecoder {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_Vtbl;
}
unsafe impl ::windows::core::Interface for WwwFormUrlDecoder {
    const IID: ::windows::core::GUID = <IWwwFormUrlDecoderRuntimeClass as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
::windows::core::interface_hierarchy!(WwwFormUrlDecoder, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for ::windows::core::InParam<Collections::IIterable<IWwwFormUrlDecoderEntry>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for ::windows::core::InParam<Collections::IVectorView<IWwwFormUrlDecoderEntry>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WwwFormUrlDecoder {}
unsafe impl ::core::marker::Sync for WwwFormUrlDecoder {}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct WwwFormUrlDecoderEntry(::windows::core::IUnknown);
impl WwwFormUrlDecoderEntry {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Name)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WwwFormUrlDecoderEntry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WwwFormUrlDecoderEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WwwFormUrlDecoderEntry {}
impl ::core::fmt::Debug for WwwFormUrlDecoderEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WwwFormUrlDecoderEntry").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoderEntry;{125e7431-f678-4e8e-b670-20a9b06c512d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_Vtbl;
}
unsafe impl ::windows::core::Interface for WwwFormUrlDecoderEntry {
    const IID: ::windows::core::GUID = <IWwwFormUrlDecoderEntry as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoderEntry";
}
::windows::core::interface_hierarchy!(WwwFormUrlDecoderEntry, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: WwwFormUrlDecoderEntry) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoderEntry) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WwwFormUrlDecoderEntry> for ::windows::core::InParam<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoderEntry) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WwwFormUrlDecoderEntry {}
unsafe impl ::core::marker::Sync for WwwFormUrlDecoderEntry {}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl ::core::marker::Copy for AsyncStatus {}
impl ::core::clone::Clone for AsyncStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AsyncStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AsyncStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for AsyncStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AsyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
impl ::core::marker::Copy for PropertyType {}
impl ::core::clone::Clone for PropertyType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PropertyType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PropertyType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PropertyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl ::core::marker::Copy for DateTime {}
impl ::core::clone::Clone for DateTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
    }
}
unsafe impl ::windows::core::Abi for DateTime {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DateTime {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.UniversalTime == other.UniversalTime
    }
}
impl ::core::cmp::Eq for DateTime {}
impl ::core::default::Default for DateTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct EventRegistrationToken {
    pub Value: i64,
}
impl ::core::marker::Copy for EventRegistrationToken {}
impl ::core::clone::Clone for EventRegistrationToken {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EventRegistrationToken").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for EventRegistrationToken {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EventRegistrationToken {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.EventRegistrationToken;i8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl ::core::marker::Copy for Point {}
impl ::core::clone::Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Point {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for Point {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Point {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for Point {}
impl ::core::default::Default for Point {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Rect {}
impl ::core::clone::Clone for Rect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Rect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for Rect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Rect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Rect {}
impl ::core::default::Default for Rect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl ::core::marker::Copy for Size {}
impl ::core::clone::Clone for Size {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Size {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for Size {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Size {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Size {}
impl ::core::default::Default for Size {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Foundation\"`*"]
pub struct TimeSpan {
    pub Duration: i64,
}
impl ::core::marker::Copy for TimeSpan {}
impl ::core::clone::Clone for TimeSpan {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TimeSpan {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
    }
}
unsafe impl ::windows::core::Abi for TimeSpan {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimeSpan {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for TimeSpan {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration
    }
}
impl ::core::cmp::Eq for TimeSpan {}
impl ::core::default::Default for TimeSpan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::convert::From<::core::time::Duration> for TimeSpan {
    fn from(value: ::core::time::Duration) -> Self {
        Self { Duration: (value.as_nanos() / 100) as i64 }
    }
}
impl ::core::convert::From<TimeSpan> for ::core::time::Duration {
    fn from(value: TimeSpan) -> Self {
        ::core::time::Duration::from_nanos((value.Duration * 100) as u64)
    }
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncActionCompletedHandler(pub ::windows::core::IUnknown);
impl AsyncActionCompletedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionCompletedHandlerBox::<F> { vtable: &AsyncActionCompletedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncAction>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
struct AsyncActionCompletedHandlerBox<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const AsyncActionCompletedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncActionCompletedHandlerBox<F> {
    const VTABLE: AsyncActionCompletedHandler_Vtbl = AsyncActionCompletedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncActionCompletedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), asyncstatus).into()
    }
}
impl ::core::clone::Clone for AsyncActionCompletedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AsyncActionCompletedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AsyncActionCompletedHandler {}
impl ::core::fmt::Debug for AsyncActionCompletedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionCompletedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for AsyncActionCompletedHandler {
    type Vtable = AsyncActionCompletedHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for AsyncActionCompletedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7);
}
unsafe impl ::windows::core::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionCompletedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncActionProgressHandler<TProgress>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TProgress: ::windows::core::RuntimeType + 'static> AsyncActionProgressHandler<TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionProgressHandlerBox::<TProgress, F> { vtable: &AsyncActionProgressHandlerBox::<TProgress, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncActionWithProgress<TProgress>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<TProgress>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), progressinfo.into().abi()).ok() }
    }
}
#[repr(C)]
struct AsyncActionProgressHandlerBox<TProgress, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncActionProgressHandler_Vtbl<TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncActionProgressHandlerBox<TProgress, F> {
    const VTABLE: AsyncActionProgressHandler_Vtbl<TProgress> = AsyncActionProgressHandler_Vtbl::<TProgress> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TProgress: ::core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncActionProgressHandler<TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), ::core::mem::transmute(&progressinfo)).into()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for AsyncActionProgressHandler<TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncActionProgressHandler<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncActionProgressHandler<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncActionProgressHandler<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionProgressHandler").field(&self.0).finish()
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for AsyncActionProgressHandler<TProgress> {
    type Vtable = AsyncActionProgressHandler_Vtbl<TProgress>;
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncActionProgressHandler<TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncActionProgressHandler<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6d844858-0cff-4590-ae89-95a5a5c8b4b8}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionProgressHandler_Vtbl<TProgress>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TProgress: ::windows::core::RuntimeType + 'static> AsyncActionWithProgressCompletedHandler<TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionWithProgressCompletedHandlerBox::<TProgress, F> { vtable: &AsyncActionWithProgressCompletedHandlerBox::<TProgress, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncActionWithProgress<TProgress>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
struct AsyncActionWithProgressCompletedHandlerBox<TProgress, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncActionWithProgressCompletedHandlerBox<TProgress, F> {
    const VTABLE: AsyncActionWithProgressCompletedHandler_Vtbl<TProgress> = AsyncActionWithProgressCompletedHandler_Vtbl::<TProgress> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TProgress: ::core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), asyncstatus).into()
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncActionWithProgressCompletedHandler<TProgress> {}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncActionWithProgressCompletedHandler<TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncActionWithProgressCompletedHandler").field(&self.0).finish()
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for AsyncActionWithProgressCompletedHandler<TProgress> {
    type Vtable = AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>;
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncActionWithProgressCompletedHandler<TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncActionWithProgressCompletedHandler<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9c029f91-cc84-44fd-ac26-0a6c4e555281}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncOperationCompletedHandler<TResult>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TResult>)
where
    TResult: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationCompletedHandlerBox::<TResult, F> { vtable: &AsyncOperationCompletedHandlerBox::<TResult, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncOperation<TResult>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
struct AsyncOperationCompletedHandlerBox<TResult, F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationCompletedHandler_Vtbl<TResult>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncOperationCompletedHandlerBox<TResult, F> {
    const VTABLE: AsyncOperationCompletedHandler_Vtbl<TResult> = AsyncOperationCompletedHandler_Vtbl::<TResult> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: ::core::marker::PhantomData::<TResult>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncOperationCompletedHandler<TResult> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), asyncstatus).into()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for AsyncOperationCompletedHandler<TResult> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TResult>)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationCompletedHandler<TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationCompletedHandler<TResult> {}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationCompletedHandler<TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationCompletedHandler").field(&self.0).finish()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for AsyncOperationCompletedHandler<TResult> {
    type Vtable = AsyncOperationCompletedHandler_Vtbl<TResult>;
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationCompletedHandler<TResult> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationCompletedHandler<TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{fcdcf02c-e5d8-4478-915a-4d90b74b83a5}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationCompletedHandler_Vtbl<TResult>
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> AsyncOperationProgressHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationProgressHandlerBox::<TResult, TProgress, F> { vtable: &AsyncOperationProgressHandlerBox::<TResult, TProgress, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncOperationWithProgress<TResult, TProgress>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::Into<::windows::core::InParam<TProgress>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), progressinfo.into().abi()).ok() }
    }
}
#[repr(C)]
struct AsyncOperationProgressHandlerBox<TResult, TProgress, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationProgressHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncOperationProgressHandlerBox<TResult, TProgress, F> {
    const VTABLE: AsyncOperationProgressHandler_Vtbl<TResult, TProgress> = AsyncOperationProgressHandler_Vtbl::<TResult, TProgress> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: ::core::marker::PhantomData::<TResult>,
        TProgress: ::core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), ::core::mem::transmute(&progressinfo)).into()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for AsyncOperationProgressHandler<TResult, TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TResult>, ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationProgressHandler<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationProgressHandler<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationProgressHandler<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationProgressHandler").field(&self.0).finish()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for AsyncOperationProgressHandler<TResult, TProgress> {
    type Vtable = AsyncOperationProgressHandler_Vtbl<TResult, TProgress>;
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationProgressHandler<TResult, TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{55690902-0aab-421a-8778-f8ce5026d758}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationProgressHandler_Vtbl<TResult, TProgress>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationWithProgressCompletedHandlerBox::<TResult, TProgress, F> { vtable: &AsyncOperationWithProgressCompletedHandlerBox::<TResult, TProgress, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, E0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IAsyncOperationWithProgress<TResult, TProgress>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), asyncinfo.try_into().map_err(|e| e.into())?.abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
struct AsyncOperationWithProgressCompletedHandlerBox<TResult, TProgress, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AsyncOperationWithProgressCompletedHandlerBox<TResult, TProgress, F> {
    const VTABLE: AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress> = AsyncOperationWithProgressCompletedHandler_Vtbl::<TResult, TProgress> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: ::core::marker::PhantomData::<TResult>,
        TProgress: ::core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&asyncinfo), asyncstatus).into()
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TResult>, ::core::marker::PhantomData::<TProgress>)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AsyncOperationWithProgressCompletedHandler").field(&self.0).finish()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    type Vtable = AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>;
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{e85df41d-6aa7-46e3-a8e2-f009d840c627}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, asyncinfo: *mut ::core::ffi::c_void, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub TResult: ::core::marker::PhantomData<TResult>,
    pub TProgress: ::core::marker::PhantomData<TProgress>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct DeferralCompletedHandler(pub ::windows::core::IUnknown);
impl DeferralCompletedHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = DeferralCompletedHandlerBox::<F> { vtable: &DeferralCompletedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct DeferralCompletedHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const DeferralCompletedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> DeferralCompletedHandlerBox<F> {
    const VTABLE: DeferralCompletedHandler_Vtbl = DeferralCompletedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<DeferralCompletedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for DeferralCompletedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeferralCompletedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeferralCompletedHandler {}
impl ::core::fmt::Debug for DeferralCompletedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeferralCompletedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for DeferralCompletedHandler {
    type Vtable = DeferralCompletedHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for DeferralCompletedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed32a372_f3c8_4faa_9cfb_470148da3888);
}
unsafe impl ::windows::core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ed32a372-f3c8-4faa-9cfb-470148da3888}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DeferralCompletedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct EventHandler<T>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
impl<T: ::windows::core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = EventHandlerBox::<T, F> { vtable: &EventHandlerBox::<T, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
        P1: ::std::convert::Into<::windows::core::InParam<T>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi(), args.into().abi()).ok() }
    }
}
#[repr(C)]
struct EventHandlerBox<T, F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    T: ::windows::core::RuntimeType + 'static,
{
    vtable: *const EventHandler_Vtbl<T>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<T: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> EventHandlerBox<T, F> {
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        T: ::core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<EventHandler<T> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for EventHandler<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<T>)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for EventHandler<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for EventHandler<T> {}
impl<T: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for EventHandler<T> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EventHandler").field(&self.0).finish()
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for EventHandler<T> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for EventHandler<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9de1c535-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_Vtbl<T>
where
    T: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, args: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub T: ::core::marker::PhantomData<T>,
}
#[doc = "*Required features: `\"Foundation\"`*"]
#[repr(transparent)]
pub struct TypedEventHandler<TSender, TResult>(pub ::windows::core::IUnknown, ::core::marker::PhantomData<TSender>, ::core::marker::PhantomData<TResult>)
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static;
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
    pub fn new<F: FnMut(&<TSender as ::windows::core::RuntimeType>::DefaultType, &<TResult as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TypedEventHandlerBox::<TSender, TResult, F> { vtable: &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<TSender>>,
        P1: ::std::convert::Into<::windows::core::InParam<TResult>>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), sender.into().abi(), args.into().abi()).ok() }
    }
}
#[repr(C)]
struct TypedEventHandlerBox<TSender, TResult, F: FnMut(&<TSender as ::windows::core::RuntimeType>::DefaultType, &<TResult as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static,
{
    vtable: *const TypedEventHandler_Vtbl<TSender, TResult>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static, F: FnMut(&<TSender as ::windows::core::RuntimeType>::DefaultType, &<TResult as ::windows::core::RuntimeType>::DefaultType) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> TypedEventHandlerBox<TSender, TResult, F> {
    const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> = TypedEventHandler_Vtbl::<TSender, TResult> {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TSender: ::core::marker::PhantomData::<TSender>,
        TResult: ::core::marker::PhantomData::<TResult>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<TypedEventHandler<TSender, TResult> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: <TSender as ::windows::core::Abi>::Abi, args: <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&args)).into()
    }
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::clone::Clone for TypedEventHandler<TSender, TResult> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), ::core::marker::PhantomData::<TSender>, ::core::marker::PhantomData::<TResult>)
    }
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::PartialEq for TypedEventHandler<TSender, TResult> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::cmp::Eq for TypedEventHandler<TSender, TResult> {}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::core::fmt::Debug for TypedEventHandler<TSender, TResult> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TypedEventHandler").field(&self.0).finish()
    }
}
unsafe impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Vtable for TypedEventHandler<TSender, TResult> {
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
}
unsafe impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for TypedEventHandler<TSender, TResult> {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<Self as ::windows::core::RuntimeType>::SIGNATURE);
}
unsafe impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for TypedEventHandler<TSender, TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9de1c534-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(<TSender as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static,
{
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: <TSender as ::windows::core::Abi>::Abi, args: <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub TSender: ::core::marker::PhantomData<TSender>,
    pub TResult: ::core::marker::PhantomData<TResult>,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
