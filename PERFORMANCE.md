| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 42.2 ± 2.5 | 38.0 | 69.8 | 16.51 ± 4.18 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.6 ± 0.6 | 1.8 | 9.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 42.9 ± 2.5 | 38.9 | 56.6 | 15.51 ± 6.23 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.8 ± 1.1 | 1.9 | 19.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 42.1 ± 1.9 | 38.5 | 57.3 | 15.97 ± 4.32 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.6 ± 0.7 | 1.8 | 8.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 192.7 ± 5.0 | 183.0 | 226.1 | 1.31 ± 0.06 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 146.7 ± 6.2 | 130.1 | 179.2 | 1.00 |

