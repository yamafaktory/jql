| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 32.2 ± 2.0 | 29.0 | 50.0 | 12.61 ± 4.57 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.6 ± 0.9 | 1.7 | 16.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.0 ± 1.8 | 28.0 | 42.2 | 13.34 ± 4.01 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.4 ± 0.7 | 1.6 | 12.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 31.7 ± 1.8 | 27.7 | 44.1 | 13.67 ± 3.39 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.3 ± 0.6 | 1.6 | 11.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 186.6 ± 5.7 | 174.9 | 217.9 | 1.22 ± 0.07 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 153.0 ± 7.0 | 135.9 | 186.2 | 1.00 |

