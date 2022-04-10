pub fn generate_keypair(&self) -> Result<Keypair, Error> {
	// 拿到当前的随机数生成算法
	let mut rng = self.resolver.resolve_rng().ok_or(InitStage::GetRngImpl)?;
	// 拿到当前的 DH 算法
	let mut dh = self.resolver.resolve_dh(&self.params.dh).ok_or(InitStage::GetDhImpl)?;
	let mut private = vec![0u8; dh.priv_len()];
	let mut public = vec![0u8; dh.pub_len()];
	// 使用随机数生成器 和 DH 生成密钥对
	dh.generate(&mut *rng);

	private.copy_from_slice(dh.privkey());
	public.copy_from_slice(dh.pubkey());

	Ok(Keypair { private, public })
}