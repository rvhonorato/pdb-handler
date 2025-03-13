type SubtitleProps = {
  text: string;
};

export const Subtitle = ({ text }: SubtitleProps) => {
  return (
    <h2 className="text-2xl font-semibold text-gray-900 relative pb-3 mb-6 inline-block">
      <span className="relative z-10">{text}</span>
      <span
        className="absolute bottom-0 left-0 w-full h-1 bg-gradient-to-r from-indigo-400 to-indigo-300 rounded-full 
                  transition-all duration-500 group-hover:scale-x-105 origin-left"
      ></span>
    </h2>
  );
};
