import {blogs} from "./../../../public/blogs/blogs";
import { ImSad } from "react-icons/im";

export default function Blogs() {

  return (
    <section className="min-h-screen px-4 sm:px-6 lg:px-12 py-12 text-white">
      <h1 className="text-3xl sm:text-4xl font-bold mb-10 text-center text-accent">Blogs</h1>
      <div className="flex flex-col gap-8 max-w-3xl mx-auto">
        {blogs.length === 0 ? (<div className="flex flex-col items-center mt-32"><ImSad className="text-5xl mb-8"/><p className="text-center">No blogs yet. Coming soon...</p></div>) : (blogs.map((blog, index) => (
          <div
            key={index}
            className="bg-black/40 border border-white/10 rounded-2xl shadow-lg p-4 sm:p-6 hover:scale-[1.02] transition-transform duration-300"
          >
            <h2 className="text-lg sm:text-2xl font-semibold mb-2">{blog.title}</h2>
            <p className="text-xs sm:text-sm text-gray-500 mb-3">{blog.date}</p>
            <p className="text-gray-400 text-sm sm:text-base mb-4">{blog.description}</p>
            <a
              href={blog.link}
              className="inline-block px-3 sm:px-4 py-2 text-xs sm:text-sm font-medium border border-blue-500 text-blue-400 rounded-lg hover:bg-blue-600/20 transition-colors"
            >
              Read More →
            </a>
          </div>
        )))}
      </div>
    </section>
  );
}