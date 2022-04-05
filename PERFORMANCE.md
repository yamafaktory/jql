| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 31.6 ± 1.2 | 31.1 | 66.9 | 16.44 ± 2.37 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.3 | 1.6 | 7.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 31.3 ± 0.2 | 30.9 | 33.6 | 17.76 ± 3.69 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 1.8 ± 0.4 | 1.4 | 8.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 31.4 ± 0.2 | 31.0 | 32.7 | 17.05 ± 4.36 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 1.8 ± 0.5 | 1.5 | 11.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 153.7 ± 1.8 | 151.8 | 173.0 | 1.25 ± 0.03 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 122.9 ± 2.0 | 119.0 | 154.4 | 1.00 |

