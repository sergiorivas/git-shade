class GitShade < Formula
  desc "CLI tool that maintains a single unified Git repository for all your project's excluded files (configs, secrets, large files)"
  homepage "https://sergiorivas@github.com/sergiorivas/git-shade"
  version "1.0.2"
  license "MIT"

  on_macos do
    if Hardware::CPU.intel?
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.2/git-shade-v1.0.2-x86_64-apple-darwin.tar.gz"
      sha256 "23cd80210e96ea49d43a9b6e4bca5c3aba196c3dbff80abcbf747e2d1e56975a"
    else
      url "https://github.com/sergiorivas/git-shade/releases/download/v1.0.2/git-shade-v1.0.2-aarch64-apple-darwin.tar.gz"
      sha256 "0f15b86b5d314eb0c406db3cd911789f206709521c9d2ab524da10325f2e8e54"
    end
  end

  def install
    bin.install "git-shade"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/git-shade --version")
  end
end
