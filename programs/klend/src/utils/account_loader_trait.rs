use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
    marker::PhantomData,
    ops::DerefMut,
};

use anchor_lang::{
    error::ErrorCode, prelude::AccountLoader, Accounts, Arbitrary, Key, Owner, Result,
    ToAccountInfos, ToAccountMetas, ZeroCopy,
};
use bytemuck::{Pod, Zeroable};
use solana_program::{vec, Ref, RefMut};

use solana_program::{account_info::AccountInfo, instruction::AccountMeta, pubkey::Pubkey};

pub trait AnyAccountLoader<'info, T> {
    fn get_mut(&self) -> Result<RefMut<T>>;
    fn get(&self) -> Result<Ref<T>>;
    fn get_pubkey(&self) -> Pubkey;
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> AnyAccountLoader<'info, T>
    for AccountLoader<'info, T>
{
    fn get_mut(&self) -> Result<RefMut<T>> {
        self.load_mut()
    }
    fn get(&self) -> Result<Ref<T>> {
        self.load()
    }

    fn get_pubkey(&self) -> Pubkey {
        self.key()
    }
}

pub struct FatAccountLoader<'info, T: ZeroCopy + Owner> {
    acc_info: AccountInfo<'info>,
    phantom: PhantomData<&'info T>,
}

impl<'info, T: ZeroCopy + Owner + fmt::Debug> fmt::Debug for FatAccountLoader<'info, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccountLoader")
            .field("acc_info", &self.acc_info)
            .field("phantom", &self.phantom)
            .finish()
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> FatAccountLoader<'info, T> {
    fn new(acc_info: &AccountInfo<'info>) -> FatAccountLoader<'info, T> {
        Self {
            acc_info: acc_info.clone(),
            phantom: PhantomData,
        }
    }

    #[inline(never)]
    pub fn try_from(acc_info: &AccountInfo<'info>) -> Result<FatAccountLoader<'info, T>> {
        // if acc_info.owner != &T::owner() {
        //     return Err(
        //         anchor_lang::error::Error::from(ErrorCode::AccountOwnedByWrongProgram)
        //             .with_pubkeys((*acc_info.owner, T::owner())),
        //     );
        // }
        // let data: &[u8] = &acc_info.try_borrow_data()?;
        // if data.len() < T::discriminator().len() {
        //     return Err(ErrorCode::AccountDiscriminatorNotFound.into());
        // }
        // if data[0..8] != T::discriminator() {
        //     return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        // }

        Ok(FatAccountLoader::new(acc_info))
    }

    #[inline(never)]
    pub fn try_from_unchecked(
        _program_id: &Pubkey,
        acc_info: &AccountInfo<'info>,
    ) -> Result<FatAccountLoader<'info, T>> {
        // if acc_info.owner != &T::owner() {
        //     return Err(
        //         anchor_lang::error::Error::from(ErrorCode::AccountOwnedByWrongProgram)
        //             .with_pubkeys((*acc_info.owner, T::owner())),
        //     );
        // }
        Ok(FatAccountLoader::new(acc_info))
    }

    pub fn load(&self) -> Result<Ref<T>> {
        // let data = self.acc_info.try_borrow_data()?;
        // if data.len() < T::discriminator().len() {
        //     return Err(ErrorCode::AccountDiscriminatorNotFound.into());
        // }

        // if data[0..8] != T::discriminator() {
        //     return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        // }

        // Ok(Ref::map(data, |data| {
        //     bytemuck::from_bytes(&data[8..std::mem::size_of::<T>() + 8])
        // }))
        Ok(Ref::new(&kani::any::<T>()))
    }

    pub fn load_mut(&self) -> Result<RefMut<T>> {
        // if !self.acc_info.is_writable {
        //     return Err(ErrorCode::AccountNotMutable.into());
        // }

        // let data = self.acc_info.try_borrow_mut_data()?;
        // if data.len() < T::discriminator().len() {
        //     return Err(ErrorCode::AccountDiscriminatorNotFound.into());
        // }

        // if data[0..8] != T::discriminator() {
        //     return Err(ErrorCode::AccountDiscriminatorMismatch.into());
        // }

        // Ok(RefMut::map(data, |data| {
        //     bytemuck::from_bytes_mut(&mut data.deref_mut()[8..std::mem::size_of::<T>() + 8])
        // }))
        Ok(RefMut::new(&kani::any::<T>()))
    }

    pub fn load_init(&self) -> Result<Ref<T>> {
        Ok(Ref::new(&kani::any::<T>()))
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> AnyAccountLoader<'info, T>
    for FatAccountLoader<'info, T>
{
    fn get_mut(&self) -> Result<RefMut<T>> {
        self.load_mut()
    }
    fn get(&self) -> Result<Ref<T>> {
        self.load()
    }

    fn get_pubkey(&self) -> Pubkey {
        self.acc_info.key()
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> Accounts<'info> for FatAccountLoader<'info, T> {
    #[inline(never)]
    fn try_accounts(
        _program_id: &Pubkey,
        accounts: &mut &[AccountInfo<'info>],
        _ix_data: &[u8],
        _bumps: &mut BTreeMap<String, u8>,
        _reallocs: &mut BTreeSet<Pubkey>,
    ) -> Result<Self> {
        if accounts.is_empty() {
            return Err(ErrorCode::AccountNotEnoughKeys.into());
        }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        let l = FatAccountLoader::try_from(account)?;
        Ok(l)
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> ToAccountMetas for FatAccountLoader<'info, T> {
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.acc_info.is_signer);
        let meta = match self.acc_info.is_writable {
            false => AccountMeta::new_readonly(*self.acc_info.key, is_signer),
            true => AccountMeta::new(*self.acc_info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> AsRef<AccountInfo<'info>>
    for FatAccountLoader<'info, T>
{
    fn as_ref(&self) -> &AccountInfo<'info> {
        &self.acc_info
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> ToAccountInfos<'info>
    for FatAccountLoader<'info, T>
{
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.acc_info.clone()]
    }
}

impl<'info, T: ZeroCopy + Owner + kani::Arbitrary> Key for FatAccountLoader<'info, T> {
    fn key(&self) -> Pubkey {
        *self.acc_info.key
    }
}

#[cfg(any(kani, feature = "kani"))]
impl<'info, T: Owner + ZeroCopy> kani::Arbitrary for FatAccountLoader<'info, T> {
    fn any() -> Self {
        Self {
            acc_info: kani::any(),
            phantom: PhantomData,
        }
    }
}
