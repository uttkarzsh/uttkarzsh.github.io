export default function Contact() {
  return (
    <section className="min-h-screen px-4 sm:px-6 lg:px-12 py-16 text-white">
      <h1 className="text-3xl sm:text-4xl font-bold mb-12 text-center">Contact Me</h1>

      {/* Social Links */}
      <div className="flex justify-center gap-6 mb-12">
        <a
          href="https://github.com/yourusername"
          target="_blank"
          rel="noopener noreferrer"
          className="p-3 border border-white/20 rounded-full hover:border-blue-500 hover:text-blue-400 transition"
        >
          <i className="fab fa-github text-xl"></i>
        </a>
        <a
          href="https://linkedin.com/in/yourusername"
          target="_blank"
          rel="noopener noreferrer"
          className="p-3 border border-white/20 rounded-full hover:border-blue-500 hover:text-blue-400 transition"
        >
          <i className="fab fa-linkedin text-xl"></i>
        </a>
        <a
          href="https://x.com/yourusername"
          target="_blank"
          rel="noopener noreferrer"
          className="p-3 border border-white/20 rounded-full hover:border-blue-500 hover:text-blue-400 transition"
        >
          <i className="fab fa-x-twitter text-xl"></i>
        </a>
      </div>

      {/* Contact Form */}
      <form
        action="mailto:your-email@example.com"
        method="POST"
        encType="text/plain"
        className="max-w-2xl mx-auto bg-black/40 border border-white/10 rounded-2xl shadow-lg p-6 sm:p-8"
      >
        <div className="mb-4">
          <label className="block text-sm mb-2">Name</label>
          <input
            type="text"
            name="name"
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-blue-500"
          />
        </div>

        <div className="mb-4">
          <label className="block text-sm mb-2">Email</label>
          <input
            type="email"
            name="email"
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-blue-500"
          />
        </div>

        <div className="mb-6">
          <label className="block text-sm mb-2">Message</label>
          <textarea
            name="message"
            rows={5}
            required
            className="w-full px-4 py-2 rounded-lg bg-black/60 border border-white/20 text-white focus:outline-none focus:border-blue-500"
          ></textarea>
        </div>

        <button
          type="submit"
          className="w-full sm:w-auto px-6 py-2 rounded-lg border border-blue-500 text-blue-400 hover:bg-blue-600/20 transition"
        >
          Send Message
        </button>
      </form>
    </section>
  );
}