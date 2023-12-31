use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
#[repr(u16)]
pub enum Item {
	Capsule = 0,
	Box = 1,
	Barrel = 2,
	Egg = 3,
	PartyBall = 4,
	BarrelCannon = 5,
	BobOmb = 6,
	MrSaturn = 7,
	HeartContainer = 8,
	MaximTomato = 9,
	Starman = 10,
	HomeRunBat = 11,
	BeamSword = 12,
	Parasol = 13,
	GreenShell1 = 14,
	RedShell1 = 15,
	RayGun = 16,
	Freezie = 17,
	Food = 18,
	ProximityMine = 19,
	Flipper = 20,
	SuperScope = 21,
	StarRod = 22,
	LipStick = 23,
	Fan = 24,
	FireFlower = 25,
	SuperMushroom = 26,
	WarpStar = 29,
	ScrewAttack = 30,
	BunnyHood = 31,
	MetalBox = 32,
	CloakingDevice = 33,
	PokeBall = 34,
	RayGunRecoil = 35,
	StarRodStar = 36,
	LipStickDust = 37,
	SuperScopeBeam = 38,
	RayGunBeam = 39,
	HammerHead = 40,
	Flower = 41,
	YoshiEgg1 = 42,
	Goomba = 43,
	Redead = 44,
	Octarok = 45,
	Ottosea = 46,
	Stone = 47,
	MarioFire = 48,
	DrMarioPill = 49,
	KirbyCutterBeam = 50,
	KirbyHammer = 51,
	FoxLaser = 54,
	FalcoLaser = 55,
	FoxShadow = 56,
	FalcoShadow = 57,
	LinkBomb = 58,
	YoungLinkBomb = 59,
	LinkBoomerang = 60,
	YoungLinkBoomerang = 61,
	LinkHookshot = 62,
	YoungLinkHookshot = 63,
	LinkArrow1 = 64,
	YoungLinkFireArrow = 65,
	NessPkFire = 66,
	NessPkFlash1 = 67,
	NessPkFlash2 = 68,
	NessPkThunder1 = 69,
	NessPkThunder2 = 70,
	NessPkThunder3 = 71,
	NessPkThunder4 = 72,
	NessPkThunder5 = 73,
	FoxBlaster = 74,
	FalcoBlaster = 75,
	LinkArrow2 = 76,
	YoungLinkArrow = 77,
	NessPkFlash3 = 78,
	SheikNeedle1 = 79,
	SheikNeedle2 = 80,
	PikachuThunder1 = 81,
	PichuThunder1 = 82,
	MarioCape = 83,
	DrMarioCape = 84,
	SheikSmoke = 85,
	YoshiEgg2 = 86,
	YoshiTongue1 = 87,
	YoshiStar = 88,
	PikachuThunder2 = 89,
	PikachuThunder3 = 90,
	PichuThunder2 = 91,
	PichuThunder3 = 92,
	SamusBomb = 93,
	SamusChargeShot = 94,
	SamusMissile = 95,
	SamusGrappleBeam = 96,
	SheikChain = 97,
	PeachTurnip = 99,
	BowserFlame = 100,
	NessBat = 101,
	NessYoyo = 102,
	PeachParasol = 103,
	PeachToad = 104,
	LuigiFire = 105,
	IceClimbersIce = 106,
	IceClimbersBlizzard = 107,
	ZeldaFire1 = 108,
	ZeldaFire2 = 109,
	PeachToadSpore = 111,
	MewtwoShadowBall = 112,
	IceClimbersUpB = 113,
	GameAndWatchPesticide = 114,
	GameAndWatchManhole = 115,
	GameAndWatchFire = 116,
	GameAndWatchParachute = 117,
	GameAndWatchTurtle = 118,
	GameAndWatchSperky = 119,
	GameAndWatchJudge = 120,
	GameAndWatchSausage = 122,
	GameAndWatchMilk = 123,
	GameAndWatchFirefighter = 124,
	MasterHandLaser = 125,
	MasterHandBullet = 126,
	CrazyHandLaser = 127,
	CrazyHandBullet = 128,
	CrazyHandBomb = 129,
	KirbyCopyMarioFire = 130,
	KirbyCopyDrMarioPill = 131,
	KirbyCopyLuigiFire = 132,
	KirbyCopyIceClimbersIce = 133,
	KirbyCopyPeachToad = 134,
	KirbyCopyToadSpore = 135,
	KirbyCopyFoxLaser = 136,
	KirbyCopyFalcoLaser = 137,
	KirbyCopyFoxBlaster = 138,
	KirbyCopyFalcoBlaster = 139,
	KirbyCopyLinkArrow1 = 140,
	KirbyCopyYoungLinkArrow1 = 141,
	KirbyCopyLinkArrow2 = 142,
	KirbyCopyYoungLinkArrow2 = 143,
	KirbyCopyMewtwoShadowBall = 144,
	KirbyCopyPkFlash = 145,
	KirbyCopyPkFlashExplosion = 146,
	KirbyCopyPikachuThunder1 = 147,
	KirbyCopyPikachuThunder2 = 148,
	KirbyCopyPichuThunder1 = 149,
	KirbyCopyPichuThunder2 = 150,
	KirbyCopySamusChargeShot = 151,
	KirbyCopySheikNeedle1 = 152,
	KirbyCopySheikNeedle2 = 153,
	KirbyCopyBowserFlame = 154,
	KirbyCopyGameAndWatchSausage = 155,
	YoshiTongue2 = 157,
	MarioLuigiCoin = 159,
	RandomPokemon = 160,
	Goldeen = 161,
	Chicorita = 162,
	Snorlax = 163,
	Blastoise = 164,
	Weezing = 165,
	Charizard = 166,
	Moltres = 167,
	Zapdos = 168,
	Articuno = 169,
	Wobbuffet = 170,
	Scizor = 171,
	Unown = 172,
	Entei = 173,
	Raikou = 174,
	Suicune = 175,
	Bellossom = 176,
	Electrode = 177,
	Lugia = 178,
	HoOh = 179,
	Ditto = 180,
	Clefairy = 181,
	Togepi = 182,
	Mew = 183,
	Celebi = 184,
	Staryu = 185,
	Chansey = 186,
	Porygon = 187,
	Cyndaquil = 188,
	Marill = 189,
	Venusaur = 190,
	ChicoritaLeaf = 191,
	BlastoiseWater = 192,
	WeezingGas1 = 193,
	WeezingGas2 = 194,
	CharizardBreath1 = 195,
	CharizardBreath2 = 196,
	CharizardBreath3 = 197,
	CharizardBreath4 = 198,
	MiniUnowns = 199,
	LugiaAeroblast1 = 200,
	LugiaAeroblast2 = 201,
	LugiaAeroblast3 = 202,
	HoOhFlame = 203,
	StaryuStar = 204,
	HealingEgg = 205,
	CyndaquilFire = 206,
	OldGoomba = 208,
	Target = 209,
	Shyguy = 210,
	Koopa1 = 211,
	Koopa2 = 212,
	LikeLike = 213,
	OldOttosea = 216,
	WhiteBear = 217,
	Klap = 218,
	GreenShell2 = 219,
	RedShell2 = 220,
	Tingle = 221,
	Apple = 225,
	HealingApple = 226,
	Tool = 230,
	Birdo = 233,
	ArwingLaser = 234,
	GreatFoxLaser = 235,
	BirdoEgg = 236,
}
