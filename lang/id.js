// Tilawa: teks Bahasa Indonesia.
// Ubah teks di antara tanda kutip. Biarkan tag HTML seperti <span>, <em> dan <a ...> tetap ada.
// Butuh tanda kutip ganda di dalam teks? Tulis \" atau pakai kutip miring “ ”.
// Key yang dihapus akan tampil dalam bahasa Inggris (dari en.js).

window.TILAWA_LANG = window.TILAWA_LANG || {};
window.TILAWA_LANG.id = {
  // Tab browser
  title: "Tilawa · 29 hari menuju khatam",
  langLabel: "Switch to English", // label screen reader untuk tombol EN/ID
  themeToDark: "Ganti ke mode gelap",   // tombol bulan/matahari
  themeToLight: "Ganti ke mode terang",

  // Navigasi
  skip: "Langsung ke jadwal",
  navSchedule: "Jadwal",
  navExcuses: "Alasan",
  navSupport: "Dukung",
  navStart: "Mulai Hari 1",

  // Hero
  breathIn: "Tarik napas",
  breathOut: "Buang napas",
  moreOpen: "Lihat kenapa jadwal ini masuk akal", // chevron di bawah hero (screen reader + tooltip)
  moreClose: "Sembunyikan",
  moonLabel: "Tap bulannya biar makin purnama", // label screen reader untuk bulan di hero
  kicker: "Trigger baca Qur’ an buat orang islam yang rutin stress dan f-up",
  h1: "<span>Bukan nggak sempat.</span> <span>Kamu cuma belum sadar.</span>",
  heroSub:
    "Kerjaan bikin burnout. Scrolling bikin brainrot. Pelariannya masih bukan Al-Qur'an? Kasian.",
  ctaPlan: "Ok. Mana rulesnya",
  laterA: "Besok aja deh", // label tombol
  laterB: "Iya. Kayak kemarin.", // label yang muncul saat hover / tap

  // Baris statistik besar (tiap <span> jadi satu baris di HP)
  stats:
    "<span>29 hari.</span> <span>114 surah.</span> <span>Jangan banyak 'tapi'.</span>",

  // Bagian screen time
  realityH: "Coba kita cek habis waktu kamu karena apa.",
  realityP:
    "Tiap bangun tidur, WhatsApp ga telat dicek, reels ga stop ditonton. Selain waktu kerja ga lepas dari hp. Tapi 45 menit sama Qur’an? “Nggak ada waktu. Aku sibuk.” Nah, mungkin itu sebab kamu <em>ODGJ</em>.",
  ayah: "“Ingatlah, hanya dengan mengingat Allah hati menjadi tenang.”",
  ayahNote:
    'Cek terjemahnya di <a href="#schedule" data-jump="9">Hari 9</a>. Spoiler.',

  // Bagian plan
  planH: "Ngajinya per surah, bukan per juz.",
  planLede:
    "Nggak ada cerita yang lupa halaman. Tiap hari selesai di akhir surah, feelnya bakal beda.",
  r1h: "Hari pertama. Big match.",
  r1p: "Al-Fatihah plus Al-Baqarah full. Sengaja ditaruh di depan. Uji mental.",
  r1s: "Hari 1 · 49 hal",   // angka kecil di bawah kartu
  r2h: "Mereka bestie.",
  r2p: "Al-Anfal &amp; At-Taubah di Hari 7, Al-Isra’ &amp; Al-Kahf di Hari 11. Mereka sepaket.",
  r2s: "Hari 7 &amp; 11",
  r3h: "Hari 15: Al-Furqan.",
  r3p: "Artinya pembeda. Purnama.",
  r3s: "Hari 15 · 8 hal",
  r4h: "Hari 16–29 makin ringan.",
  r4p: "Sekitar 17 halaman sehari. Waktunya setara satu scrolling tiktok.",
  r4s: "±17 hal / hari",

  // Bagian jadwal
  schedH: "Jadwal 29 hari",
  schedLede: "Tap kartunya atau + tiap selesai satu halaman.",
  note: "Tiap hari yang selesai bulannya terus berubah. Progres ke save di browser kamu.",
  day: "Hari", // label di atas nomor hari di tiap kartu
  surah: "Surah ", // satu surah, mis. "Surah 3"
  surahs: "Surah ", // rentang, mis. "Surah 8–9"
  done: "Selesai", // cadangan kalau ada hari yang kosong di bawah
  pagesUnit: "hal",       // di kartu: "12/49 hal"
  pagesShort: "Hal",      // di kartu: "Hal 1–49"
  pagePlus: "Tambah satu halaman",
  pageMinus: "Kurangi satu halaman",
  // Kata di dalam kartu, acak setiap tap: menenangkan, plus ajakan (agak sarkas) menjauhi sosmed & notifikasi.
  // {left} = sisa halaman hari itu.
  cheer: [
    "Tarik napas. Notifikasinya nggak ke mana-mana.",
    "Hp-nya taruh dulu. Dia nggak bakal ngambek.",
    "Tenang. Grup WA bisa hidup tanpa kamu.",
    "Reels nggak bakal kangen kamu. Lanjut.",
    "Hati lagi adem. TikTok-nya jangan dibuka.",
    "Mode pesawat: fitur paling underrated.",
    "Dunia masih berisik. Di sini tenang.",
    "Timeline nggak butuh kamu sekarang.",
    "Satu halaman lagi, satu scroll lebih sedikit.",
    "FOMO-nya nanti aja. Damai dulu.",
    "Tinggal {left} halaman. Story orang bisa nunggu.",
    "Notif merah itu bisa sabar. Kamu juga.",
    "Pelan aja. Di sini nggak ada yang nge-like.",
    "Detox paling murah: gratis.",
    "Algoritma kehilangan kamu. Bagus.",
    "{left} halaman lagi. Dunia aman, kok.",
  ],
  // Tulisan di kartu setelah hari itu dicentang. Hari 1 tetap serius.
  doneDays: [
    "Alhamdulillah. Tuntas.", // Hari 1
    "Lho, lanjut beneran?", // Hari 2
    "Tumben konsisten.", // Hari 3
    "Siapa kamu sebenarnya?", // Hari 4
    "Grup kantor kalah.", // Hari 5
    "Masih hidup, kan?", // Hari 6
    "Seminggu. Boleh pamer dikit.", // Hari 7
    "Nggak nyangka sih.", // Hari 8
    "Hati tenteram? Kan.", // Hari 9
    "Dua digit. Gaya.", // Hari 10
    "Paket hemat, lunas.", // Hari 11
    "Rebahan kalah telak.", // Hari 12
    "Ini bukan kebetulan lagi.", // Hari 13
    "Hampir setengah. Tahan.", // Hari 14
    "Lewat zona nyerah.", // Hari 15
    "Turunan. Nikmati.", // Hari 16
    "Ternyata konsisten itu bisa.", // Hari 17
    "Notifikasi? Nanti aja.", // Hari 18
    "Sejak kapan kamu rajin?", // Hari 19
    "Dua puluh. Serius?", // Hari 20
    "Tiga minggu. Ngeri.", // Hari 21
    "Dikit lagi. Jangan belagu.", // Hari 22
    "Doomscroll, pensiun.", // Hari 23
    "Lima lagi. Jangan oleng.", // Hari 24
    "Surah pendek, semangat panjang.", // Hari 25
    "Tiga lagi. Fokus.", // Hari 26
    "Dua lagi. Jangan kumat.", // Hari 27
    "Besok final. Tidur cepat.", // Hari 28
    "Khatam. Siapa sangka.", // Hari 29
  ],
  reset: "Reset progres",
  resetArmed: "Tap sekali lagi buat reset 29 hari",
  skipWarn: "Jangan di skip surat ini", // muncul di hari yang mau dilompati
  carouselOne: "Hari {from} dari 29",          // posisi carousel di HP
  carouselMany: "Hari {from}–{to} dari 29",    // posisi carousel di tablet/desktop
  carouselPrev: "Hari sebelumnya",
  carouselNext: "Hari berikutnya",
  undoLast: "Batalinnya dari belakang: Hari {n} dulu.", // saat batalkan hari sebelumnya; {n} = hari terakhir yang selesai
  later: "Iya. Kayak kemarin.", // dibacakan screen reader saat "Besok aja deh" di-tap

  // Pesan progres: [0 hari, 1–14 hari, 15–28 hari, semua 29]
  msg: [
    "Masih nol. Ayo mulai.",
    "Oke, kelihatan usahanya.",
    "Lewat setengah jalan. Nanggung kalau skip.",
    "Khatam. Alhamdulillah. Sekarang ulang lagi.",
  ],
  // Selama Hari 1 (big match): positif banget. Kalimat di samping angka progres besar, ganti saat satu hari selesai.
  progressDay1: [
    "Langkah pertama selalu yang paling berat. Kamu sudah mulai.",
    "Big match berjalan. Bismillah, terus.",
    "Al-Baqarah memang panjang. Kamu lebih kuat.",
    "Hari pertama. Niatmu sudah bagus.",
    "Pelan tapi pasti. Kamu di jalan yang benar.",
    "Setiap halaman dihitung. Semangat.",
  ],
  // Hari 2 dan seterusnya: semangat, tapi sedikit sarkas.
  progress: [
    "Wah, tumben belum buka TikTok.",
    "Grup WA bisa nunggu. Beneran.",
    "Lanjut. Reels nggak bakal kangen kamu.",
    "Rajin juga ternyata. Siapa sangka.",
    "Notifikasi biar sabar dulu.",
    "Ini bukan mimpi. Kamu beneran baca.",
    "Rebahan bisa nanti. Lanjut.",
    "Jangan tiba-tiba sibuk, ya.",
    "Kamu pernah begadang buat yang lebih nggak penting.",
    "Lihat kamu, konsisten. Aneh tapi bagus.",
  ],

  // Tag di kartu hari tertentu
  tags: {
    boss: "Big match", // Hari 1
    pack: "Paket hemat", // Hari 7 dan Hari 11
    half: "Setengah jalan. Jangan berhenti disini.", // Hari 15
    end: "Khatam. Boleh flexing ke mamah.", // Hari 29
  },

  // Bagian alasan: q = alasannya, a = bantahannya
  excH: "Alasan kamu gini, kan?",
  e1q: "Aku lagi stres banget.",
  e1a: "Lho, justru itu intinya.",
  e2q: "Ini ada deadline.",
  e2a: "Alesannya klasik.",
  e3q: "Besok ada ujian.",
  e3a: "Bukannya ngaji bikin otak fresh?",
  e4q: "Politik bikin aku gila.",
  e4a: "Dari zaman Fir'aun Musa juga gitu. Buka Qur'annya.",
  e5q: "Aku bolong sehari.",
  e5a: "Tambal di hari selanjutnya. Disiplin bikin tenang.",
  e6q: "Hari 1 kepanjangan.",
  e6a: "Bagi Al-Baqarah jadi dua hari, terus gabung dua hari yang ringan di belakang.",

  // Penutup
  finalH: "Mulai hari ini. Bukan Senin.",
  finalLede: "Bukan habis sprint. Bukan habis UAS. Bukan habis pemilu.",
  finalCta: "Iya, iya. Hari 1.",
  finalMicro:
    "45 menit. Kamu pernah buang lebih banyak buat hal yang lebih nggak penting.",
  shareCta: "Ajak temanmu", // tombol salin link
  shareDone: "Tersalin",
  shareFail: "Gagal menyalin. Salin dari address bar aja.",

  // Bagian dukungan (Saweria)
  supportH: "Bantu bulannya tetap nyala.",
  supportP: "Web ini gratis, ga ada biaya bulanan & iklan. Akan tetap begitu.",
  supportCta: "Dukung lewat Saweria",
  supportNote: "Scan QR-nya atau tap tombolnya.",

  // Footer
  footer:
    "Dibuat untuk orang capek yang terus bilang “nanti”. Kalau itu kamu, tenang, yang bikin web juga gitu.",
};
