import { projects } from "./../../../public/projects/projects";
import { FaGithub } from "react-icons/fa";
import { FiExternalLink } from "react-icons/fi";
import Image from "next/image";

export default function Projects() {
  return (
    <section className="min-h-screen px-4 sm:px-6 lg:px-12 py-12 text-white">
      <h1 className="text-3xl sm:text-4xl font-bold mb-10 text-center text-accent">Projects</h1>
      <div className="flex flex-col gap-10 max-w-5xl mx-auto">
        {projects.map((project, index) => (
          <div
            key={index}
            className="bg-black/40 border border-white/10 rounded-2xl shadow-lg overflow-hidden hover:scale-[1.02] transition-transform duration-300"
          >
            {/* Project Image */}
            <Image
              src={project.image}
              alt={project.title}
              className="w-full h-48 sm:h-64 md:h-72 object-cover"
            />

            {/* Project Content */}
            <div className="p-4 sm:p-6">
              <h2 className="text-xl sm:text-2xl font-semibold mb-2">{project.title}</h2>
              <p className="text-gray-400 text-sm sm:text-base mb-4">{project.description}</p>

              {/* Tech stack */}
              <div className="flex flex-wrap gap-2 mb-4">
                {project.tech.map((t, i) => (
                  <span
                    key={i}
                    className="px-2 sm:px-3 py-1 text-xs sm:text-sm bg-blue-600/20 border border-accent rounded-full"
                  >
                    {t}
                  </span>
                ))}
              </div>

              {/* Buttons */}
              <div className="flex gap-3">
                {/* GitHub Button */}
                <a
                  href={project.github}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="flex items-center gap-2 px-2 py-1 text-xs xl:text-xs border border-white text-white hover:bg-white hover:text-black transition"
                >
                  <FaGithub className="text-lg" />
                  GitHub
                </a>

                {/* Live Demo Button */}
                {project.live && (
                  <a
                    href={project.live}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="flex items-center gap-2 px-4 py-2 text-sm sm:text-base rounded-lg border border-green-500 text-green-400 hover:bg-green-600/20 transition"
                  >
                    <FiExternalLink className="text-lg" />
                    Live Demo
                  </a>
                )}
              </div>
            </div>
          </div>
        ))}
      </div>
    </section>
  );
}